//! Ivy Language Server Protocol (LSP) implementation.
//!
//! Provides IDE features for ivy scenario files:
//! - Diagnostics (validation errors and warnings)
//! - Go to Definition (label jumps)
//! - Find References (label references)
//! - Completion (keywords, labels, assets)
//! - Hover (documentation)

use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Result;
use tokio::sync::RwLock;
use tower_lsp::jsonrpc;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use ivy::scenario::parser::parse_scenario;
use ivy::scenario::position::PositionMap;
use ivy::scenario::validator::{Severity, validate_scenario};

/// Document state stored by the server.
struct DocumentState {
    text: String,
    position_map: PositionMap,
}

/// Ivy Language Server.
struct IvyLanguageServer {
    client: Client,
    documents: Arc<RwLock<HashMap<Url, DocumentState>>>,
}

impl IvyLanguageServer {
    fn new(client: Client) -> Self {
        Self {
            client,
            documents: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Validate a document and publish diagnostics.
    async fn validate_and_publish(&self, uri: &Url, text: &str) {
        let diagnostics = self.get_diagnostics(text);
        self.client
            .publish_diagnostics(uri.clone(), diagnostics, None)
            .await;
    }

    /// Get diagnostics from scenario text.
    fn get_diagnostics(&self, text: &str) -> Vec<Diagnostic> {
        let position_map = PositionMap::from_yaml(text);

        match parse_scenario(text) {
            Ok(scenario) => {
                let result = validate_scenario(&scenario);
                result
                    .issues
                    .iter()
                    .map(|issue| {
                        let range = if let Some(index) = issue.command_index {
                            position_map
                                .get_command_position(index)
                                .map(|pos| Range {
                                    start: Position::new(pos.line, pos.column),
                                    end: Position::new(pos.line, pos.column + 1),
                                })
                                .unwrap_or_default()
                        } else {
                            Range::default()
                        };

                        Diagnostic {
                            range,
                            severity: Some(match issue.severity {
                                Severity::Error => DiagnosticSeverity::ERROR,
                                Severity::Warning => DiagnosticSeverity::WARNING,
                            }),
                            source: Some("ivy".to_string()),
                            message: issue.message.clone(),
                            ..Default::default()
                        }
                    })
                    .collect()
            }
            Err(e) => {
                // Parse error - try to extract line number from error message.
                let (line, message) = extract_parse_error_position(&e.to_string());
                vec![Diagnostic {
                    range: Range {
                        start: Position::new(line, 0),
                        end: Position::new(line, 0),
                    },
                    severity: Some(DiagnosticSeverity::ERROR),
                    source: Some("ivy".to_string()),
                    message,
                    ..Default::default()
                }]
            }
        }
    }
}

/// Extract line number from serde_yaml error messages.
fn extract_parse_error_position(error: &str) -> (u32, String) {
    // serde_yaml errors often contain "at line X column Y".
    if let Some(pos) = error.find("at line ") {
        let after_line = &error[pos + 8..];
        if let Some(space_pos) = after_line.find(' ')
            && let Ok(line) = after_line[..space_pos].parse::<u32>()
        {
            return (line.saturating_sub(1), error.to_string());
        }
    }
    (0, error.to_string())
}

#[tower_lsp::async_trait]
impl LanguageServer for IvyLanguageServer {
    async fn initialize(&self, _: InitializeParams) -> jsonrpc::Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        open_close: Some(true),
                        change: Some(TextDocumentSyncKind::FULL),
                        save: Some(TextDocumentSyncSaveOptions::SaveOptions(SaveOptions {
                            include_text: Some(true),
                        })),
                        ..Default::default()
                    },
                )),
                definition_provider: Some(OneOf::Left(true)),
                references_provider: Some(OneOf::Left(true)),
                completion_provider: Some(CompletionOptions {
                    trigger_characters: Some(vec![":".to_string(), " ".to_string()]),
                    ..Default::default()
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "ivy-lsp".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "ivy-lsp initialized")
            .await;
    }

    async fn shutdown(&self) -> jsonrpc::Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;

        // Store document state.
        {
            let mut docs = self.documents.write().await;
            docs.insert(
                uri.clone(),
                DocumentState {
                    position_map: PositionMap::from_yaml(&text),
                    text: text.clone(),
                },
            );
        }

        self.validate_and_publish(&uri, &text).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        if let Some(change) = params.content_changes.into_iter().next() {
            let text = change.text;

            // Update document state.
            {
                let mut docs = self.documents.write().await;
                docs.insert(
                    uri.clone(),
                    DocumentState {
                        position_map: PositionMap::from_yaml(&text),
                        text: text.clone(),
                    },
                );
            }

            self.validate_and_publish(&uri, &text).await;
        }
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        let uri = params.text_document.uri;
        if let Some(text) = params.text {
            self.validate_and_publish(&uri, &text).await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;
        let mut docs = self.documents.write().await;
        docs.remove(&uri);
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> jsonrpc::Result<Option<GotoDefinitionResponse>> {
        let uri = &params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        let docs = self.documents.read().await;
        let Some(doc) = docs.get(uri) else {
            return Ok(None);
        };

        // Find the label at the cursor position.
        let Some(label) = find_label_at_position(&doc.text, position) else {
            return Ok(None);
        };

        // Find the label definition.
        let Some(def_pos) = doc.position_map.get_label_position(&label) else {
            return Ok(None);
        };

        Ok(Some(GotoDefinitionResponse::Scalar(Location {
            uri: uri.clone(),
            range: Range {
                start: Position::new(def_pos.line, def_pos.column),
                end: Position::new(def_pos.line, def_pos.column + label.len() as u32 + 7), // "label: " + label
            },
        })))
    }

    async fn references(&self, params: ReferenceParams) -> jsonrpc::Result<Option<Vec<Location>>> {
        let uri = &params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;

        let docs = self.documents.read().await;
        let Some(doc) = docs.get(uri) else {
            return Ok(None);
        };

        // Find the label at the cursor position (could be definition or reference).
        let label = find_label_at_position(&doc.text, position)
            .or_else(|| find_label_definition_at_position(&doc.text, position));

        let Some(label) = label else {
            return Ok(None);
        };

        // Find all references to this label.
        let Some(refs) = doc.position_map.get_label_references(&label) else {
            return Ok(None);
        };

        let locations: Vec<Location> = refs
            .iter()
            .map(|pos| Location {
                uri: uri.clone(),
                range: Range {
                    start: Position::new(pos.line, pos.column),
                    end: Position::new(pos.line, pos.column + label.len() as u32 + 6), // "jump: " + label
                },
            })
            .collect();

        if locations.is_empty() {
            Ok(None)
        } else {
            Ok(Some(locations))
        }
    }

    async fn completion(
        &self,
        params: CompletionParams,
    ) -> jsonrpc::Result<Option<CompletionResponse>> {
        let uri = &params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;

        let docs = self.documents.read().await;
        let Some(doc) = docs.get(uri) else {
            return Ok(None);
        };

        let context = determine_completion_context(&doc.text, position);

        let items = match context {
            CompletionContext::YamlKey => get_yaml_key_completions(),
            CompletionContext::LabelReference => get_label_completions(&doc.position_map),
            CompletionContext::CharPosition => get_char_position_completions(),
            CompletionContext::Easing => get_easing_completions(),
            CompletionContext::None => return Ok(None),
        };

        Ok(Some(CompletionResponse::Array(items)))
    }

    async fn hover(&self, params: HoverParams) -> jsonrpc::Result<Option<Hover>> {
        let position = params.text_document_position_params.position;
        let uri = &params.text_document_position_params.text_document.uri;

        let docs = self.documents.read().await;
        let Some(doc) = docs.get(uri) else {
            return Ok(None);
        };

        // Find the YAML key at the cursor position.
        let Some(key) = find_yaml_key_at_position(&doc.text, position) else {
            return Ok(None);
        };

        let Some(documentation) = get_field_documentation(&key) else {
            return Ok(None);
        };

        Ok(Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: documentation.to_string(),
            }),
            range: None,
        }))
    }
}

/// Find a label reference (jump target) at the given position.
fn find_label_at_position(text: &str, position: Position) -> Option<String> {
    let lines: Vec<&str> = text.lines().collect();
    let line = lines.get(position.line as usize)?;

    // Check if this line contains a jump reference.
    if let Some(jump_pos) = line.find("jump:") {
        let value_start = jump_pos + 5;
        let value = line[value_start..].trim();
        let label = value.trim_matches(|c| c == '"' || c == '\'');
        if !label.is_empty() {
            return Some(label.to_string());
        }
    }

    None
}

/// Find a label definition at the given position.
fn find_label_definition_at_position(text: &str, position: Position) -> Option<String> {
    let lines: Vec<&str> = text.lines().collect();
    let line = lines.get(position.line as usize)?;

    // Check if this line contains a label definition.
    if let Some(label_pos) = line.find("label:") {
        let value_start = label_pos + 6;
        let value = line[value_start..].trim();
        let label = value.trim_matches(|c| c == '"' || c == '\'');
        if !label.is_empty() {
            return Some(label.to_string());
        }
    }

    None
}

/// Find the YAML key at the given position.
fn find_yaml_key_at_position(text: &str, position: Position) -> Option<String> {
    let lines: Vec<&str> = text.lines().collect();
    let line = lines.get(position.line as usize)?;
    let trimmed = line.trim().trim_start_matches("- ");

    // Find key: value pattern.
    if let Some(colon_pos) = trimmed.find(':') {
        let key = trimmed[..colon_pos].trim();
        if !key.is_empty() {
            return Some(key.to_string());
        }
    }

    None
}

/// Completion context types.
#[derive(Debug)]
enum CompletionContext {
    YamlKey,
    LabelReference,
    CharPosition,
    Easing,
    None,
}

/// Determine what kind of completion to provide based on cursor position.
fn determine_completion_context(text: &str, position: Position) -> CompletionContext {
    let lines: Vec<&str> = text.lines().collect();
    let Some(line) = lines.get(position.line as usize) else {
        return CompletionContext::None;
    };

    let trimmed = line.trim();

    // Check for specific field contexts.
    if trimmed.starts_with("jump:") || trimmed.contains("jump:") {
        return CompletionContext::LabelReference;
    }
    if trimmed.starts_with("char_pos:") || trimmed.contains("char_pos:") {
        return CompletionContext::CharPosition;
    }
    if trimmed.starts_with("easing:") || trimmed.contains("easing:") {
        return CompletionContext::Easing;
    }

    // Check if we're at the start of a new command (after "- ").
    if trimmed.starts_with("- ") || trimmed == "-" {
        return CompletionContext::YamlKey;
    }

    // Check if we're in a command block (indented after "- ").
    let indent = line.len() - line.trim_start().len();
    if indent >= 2 && !trimmed.is_empty() {
        return CompletionContext::YamlKey;
    }

    CompletionContext::None
}

/// Get completions for YAML keys.
fn get_yaml_key_completions() -> Vec<CompletionItem> {
    let keys = [
        ("text", "Display text (supports localization)"),
        ("speaker", "Character name speaking"),
        ("label", "Define a jump target label"),
        ("jump", "Unconditional jump to a label"),
        ("background", "Background image path"),
        ("character", "Character sprite image path"),
        ("char_pos", "Character position (left/center/right)"),
        ("choices", "Present choices to the player"),
        ("bgm", "Background music file path"),
        ("se", "Sound effect file path"),
        ("voice", "Voice audio file path"),
        ("transition", "Screen transition effect"),
        ("shake", "Screen shake effect"),
        ("wait", "Wait for specified seconds"),
        ("nvl", "Toggle NVL (novel) mode"),
        ("nvl_clear", "Clear NVL text buffer"),
        ("set", "Set a variable value"),
        ("if_cond", "Conditional jump based on variable"),
        ("input", "Get text input from player"),
        ("camera", "Camera pan/zoom/tilt effect"),
        ("ambient", "Ambient audio layers"),
        ("video", "Play a video file"),
        ("video_bg", "Use video as background"),
        ("particle", "Particle effect"),
        ("cinematic_bars", "Show/hide cinematic letterbox"),
    ];

    keys.iter()
        .map(|(key, doc)| CompletionItem {
            label: format!("{}:", key),
            kind: Some(CompletionItemKind::PROPERTY),
            detail: Some(doc.to_string()),
            insert_text: Some(format!("{}: ", key)),
            insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
            ..Default::default()
        })
        .collect()
}

/// Get completions for label references.
fn get_label_completions(position_map: &PositionMap) -> Vec<CompletionItem> {
    position_map
        .labels
        .keys()
        .map(|label| CompletionItem {
            label: label.clone(),
            kind: Some(CompletionItemKind::REFERENCE),
            detail: Some("Label".to_string()),
            ..Default::default()
        })
        .collect()
}

/// Get completions for char_pos values.
fn get_char_position_completions() -> Vec<CompletionItem> {
    ["left", "center", "right"]
        .iter()
        .map(|pos| CompletionItem {
            label: pos.to_string(),
            kind: Some(CompletionItemKind::ENUM_MEMBER),
            ..Default::default()
        })
        .collect()
}

/// Get completions for easing functions.
fn get_easing_completions() -> Vec<CompletionItem> {
    [
        "linear",
        "ease_in_quad",
        "ease_out_quad",
        "ease_in_out_quad",
        "ease_in_cubic",
        "ease_out_cubic",
        "ease_in_out_cubic",
        "ease_in_quart",
        "ease_out_quart",
        "ease_in_out_quart",
        "ease_in_sine",
        "ease_out_sine",
        "ease_in_out_sine",
        "ease_in_expo",
        "ease_out_expo",
        "ease_in_out_expo",
    ]
    .iter()
    .map(|e| CompletionItem {
        label: e.to_string(),
        kind: Some(CompletionItemKind::ENUM_MEMBER),
        ..Default::default()
    })
    .collect()
}

/// Get documentation for a field.
fn get_field_documentation(field: &str) -> Option<&'static str> {
    match field {
        "text" => Some(
            "**text** - Display text\n\n\
            Text to show in the text box. Supports:\n\
            - `{color:red}text{/color}` - Colored text\n\
            - `{ruby:漢字:かんじ}` - Ruby (furigana)\n\
            - `{var:name}` - Variable interpolation\n\n\
            ```yaml\ntext: \"Hello, world!\"\n```",
        ),
        "speaker" => Some(
            "**speaker** - Character name\n\n\
            Name displayed above the text box.\n\n\
            ```yaml\nspeaker: \"Alice\"\ntext: \"Hello!\"\n```",
        ),
        "label" => Some(
            "**label** - Jump target\n\n\
            Define a label that can be jumped to with `jump:`.\n\n\
            ```yaml\nlabel: ending\ntext: \"The End\"\n```",
        ),
        "jump" => Some(
            "**jump** - Unconditional jump\n\n\
            Jump to a labeled command.\n\n\
            ```yaml\njump: ending\n```",
        ),
        "background" => Some(
            "**background** - Background image\n\n\
            Set the background image.\n\
            - Omit to keep previous background\n\
            - Use `\"\"` to clear\n\n\
            ```yaml\nbackground: \"assets/bg.png\"\n```",
        ),
        "character" => Some(
            "**character** - Character sprite\n\n\
            Display a character sprite.\n\
            - Omit to keep previous character\n\
            - Use `\"\"` to clear\n\n\
            ```yaml\ncharacter: \"assets/char.png\"\nchar_pos: center\n```",
        ),
        "char_pos" => Some(
            "**char_pos** - Character position\n\n\
            Position of the character sprite.\n\
            Values: `left`, `center`, `right`\n\n\
            ```yaml\nchar_pos: center\n```",
        ),
        "choices" => Some(
            "**choices** - Player choices\n\n\
            Present choices to the player.\n\n\
            ```yaml\nchoices:\n  - label: \"Go left\"\n    jump: left_path\n  - label: \"Go right\"\n    jump: right_path\n```",
        ),
        "bgm" => Some(
            "**bgm** - Background music\n\n\
            Play background music (loops by default).\n\
            Use `\"\"` to stop.\n\n\
            ```yaml\nbgm: \"assets/music.ogg\"\n```",
        ),
        "se" => Some(
            "**se** - Sound effect\n\n\
            Play a sound effect once.\n\n\
            ```yaml\nse: \"assets/click.ogg\"\n```",
        ),
        "transition" => Some(
            "**transition** - Screen transition\n\n\
            Apply a transition effect.\n\
            Types: `fade`, `wipe`, `slide`, `pixelate`, `iris`, `blinds`\n\n\
            ```yaml\ntransition:\n  type: fade\n  duration: 1.0\n```",
        ),
        "shake" => Some(
            "**shake** - Screen shake\n\n\
            Apply a screen shake effect.\n\n\
            ```yaml\nshake:\n  intensity: 10.0\n  duration: 0.5\n```",
        ),
        "nvl" => Some(
            "**nvl** - NVL mode toggle\n\n\
            Switch between ADV (text box) and NVL (full screen) modes.\n\n\
            ```yaml\nnvl: true  # Enter NVL mode\nnvl: false # Return to ADV mode\n```",
        ),
        "camera" => Some(
            "**camera** - Camera effects\n\n\
            Apply camera pan, zoom, or tilt.\n\n\
            ```yaml\ncamera:\n  zoom: 1.5\n  pan: { x: 100, y: 0 }\n  duration: 1.0\n```",
        ),
        _ => None,
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(IvyLanguageServer::new);
    Server::new(stdin, stdout, socket).serve(service).await;

    Ok(())
}
