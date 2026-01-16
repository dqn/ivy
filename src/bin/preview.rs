//! Real-time preview server for ivy scenarios.
//!
//! Provides a WebSocket server for editor integration and an HTTP server
//! for serving the preview HTML interface.
//!
//! Usage:
//!   ivy-preview <scenario.yaml>
//!   ivy-preview --port 3030 <scenario.yaml>

use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::process::ExitCode;
use std::sync::mpsc::{Sender, channel};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use ivy::i18n::{LocalizedString, Translations};
use ivy::scenario::{Scenario, parse_scenario};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use serde::Serialize;
use tungstenite::{Message, accept};

/// Preview state sent to the client.
#[derive(Clone, Serialize)]
struct PreviewState {
    title: String,
    command_index: usize,
    total_commands: usize,
    text: Option<String>,
    speaker: Option<String>,
    background: Option<String>,
    character: Option<String>,
    char_pos: Option<String>,
    choices: Vec<ChoiceInfo>,
    variables: HashMap<String, String>,
    labels: Vec<String>,
    current_label: Option<String>,
    nvl_mode: bool,
}

#[derive(Clone, Serialize)]
struct ChoiceInfo {
    label: String,
    jump: Option<String>,
}

/// Message types for WebSocket communication.
#[derive(Serialize)]
#[serde(tag = "type")]
#[allow(dead_code)]
enum WsMessage {
    #[serde(rename = "state")]
    State(Box<PreviewState>),
    #[serde(rename = "reload")]
    Reload { scenario: String },
    #[serde(rename = "error")]
    Error { message: String },
}

fn print_usage() {
    eprintln!("ivy-preview - Real-time scenario preview server");
    eprintln!();
    eprintln!("Usage:");
    eprintln!("  ivy-preview <scenario.yaml>        Start preview server");
    eprintln!("  ivy-preview --port 3030 <file>     Use custom port (default: 3000)");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  -h, --help    Show this help message");
    eprintln!("  --port <n>    HTTP server port (WebSocket uses port+1)");
}

fn resolve_localized(s: &LocalizedString) -> String {
    let empty_translations = Translations::new();
    s.resolve("en", &empty_translations)
}

fn build_preview_state(
    scenario: &Scenario,
    index: usize,
    variables: &HashMap<String, String>,
) -> PreviewState {
    let commands = &scenario.script;
    let total = commands.len();
    let idx = index.min(total.saturating_sub(1));

    // Collect all labels in the scenario
    let labels: Vec<String> = commands
        .iter()
        .filter_map(|cmd| cmd.label.clone())
        .collect();

    // Find current label
    let current_label = commands
        .iter()
        .take(idx + 1)
        .rev()
        .find_map(|cmd| cmd.label.clone());

    // Build visual state by scanning commands up to current index
    let mut background: Option<String> = None;
    let mut character: Option<String> = None;
    let mut char_pos: Option<String> = None;
    let mut nvl_mode = false;

    for cmd in commands.iter().take(idx + 1) {
        if let Some(ref bg) = cmd.background {
            if bg.is_empty() {
                background = None;
            } else {
                background = Some(bg.clone());
            }
        }
        if let Some(ref ch) = cmd.character {
            if ch.is_empty() {
                character = None;
            } else {
                character = Some(ch.clone());
            }
        }
        if let Some(ref pos) = cmd.char_pos {
            char_pos = Some(format!("{:?}", pos).to_lowercase());
        }
        if let Some(nvl) = cmd.nvl {
            nvl_mode = nvl;
        }
    }

    let current_cmd = commands.get(idx);
    let text = current_cmd
        .and_then(|c| c.text.as_ref())
        .map(resolve_localized);
    let speaker = current_cmd
        .and_then(|c| c.speaker.as_ref())
        .map(resolve_localized);
    let choices: Vec<ChoiceInfo> = current_cmd
        .and_then(|c| c.choices.as_ref())
        .map(|choices| {
            choices
                .iter()
                .map(|c| ChoiceInfo {
                    label: resolve_localized(&c.label),
                    jump: Some(c.jump.clone()),
                })
                .collect()
        })
        .unwrap_or_default();

    PreviewState {
        title: scenario.title.clone(),
        command_index: idx,
        total_commands: total,
        text,
        speaker,
        background,
        character,
        char_pos,
        choices,
        variables: variables.clone(),
        labels,
        current_label,
        nvl_mode,
    }
}

fn generate_html(_http_port: u16, ws_port: u16) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>ivy Preview</title>
    <style>
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: #1a1a2e;
            color: #eee;
            min-height: 100vh;
            display: flex;
            flex-direction: column;
        }}
        header {{
            background: #16213e;
            padding: 12px 20px;
            display: flex;
            justify-content: space-between;
            align-items: center;
            border-bottom: 1px solid #0f3460;
        }}
        header h1 {{ font-size: 18px; color: #e94560; }}
        .status {{ font-size: 12px; color: #888; }}
        .status.connected {{ color: #4ade80; }}
        main {{
            flex: 1;
            display: flex;
            gap: 20px;
            padding: 20px;
        }}
        .preview-area {{
            flex: 2;
            background: #16213e;
            border-radius: 8px;
            overflow: hidden;
            display: flex;
            flex-direction: column;
        }}
        .scene {{
            flex: 1;
            position: relative;
            background: #0a0a14;
            display: flex;
            align-items: center;
            justify-content: center;
            min-height: 300px;
        }}
        .scene .background {{
            position: absolute;
            inset: 0;
            background-size: cover;
            background-position: center;
            opacity: 0.8;
        }}
        .scene .character {{
            position: relative;
            max-height: 80%;
            z-index: 1;
        }}
        .scene .no-image {{
            color: #555;
            font-size: 14px;
        }}
        .textbox {{
            background: rgba(0,0,0,0.85);
            padding: 20px;
            min-height: 120px;
            border-top: 2px solid #e94560;
        }}
        .textbox.nvl {{
            position: absolute;
            inset: 0;
            min-height: auto;
            overflow-y: auto;
        }}
        .speaker {{
            color: #e94560;
            font-weight: bold;
            margin-bottom: 8px;
        }}
        .text {{
            line-height: 1.8;
            white-space: pre-wrap;
        }}
        .choices {{
            margin-top: 12px;
            display: flex;
            flex-direction: column;
            gap: 8px;
        }}
        .choice {{
            background: #0f3460;
            padding: 10px 16px;
            border-radius: 4px;
            cursor: pointer;
            transition: background 0.2s;
        }}
        .choice:hover {{ background: #e94560; }}
        .sidebar {{
            flex: 1;
            display: flex;
            flex-direction: column;
            gap: 16px;
            max-width: 300px;
        }}
        .panel {{
            background: #16213e;
            border-radius: 8px;
            padding: 16px;
        }}
        .panel h2 {{
            font-size: 14px;
            color: #e94560;
            margin-bottom: 12px;
            text-transform: uppercase;
            letter-spacing: 1px;
        }}
        .nav-buttons {{
            display: flex;
            gap: 8px;
            margin-bottom: 12px;
        }}
        .nav-buttons button {{
            flex: 1;
            padding: 10px;
            background: #0f3460;
            border: none;
            border-radius: 4px;
            color: #eee;
            cursor: pointer;
            font-size: 16px;
        }}
        .nav-buttons button:hover {{ background: #e94560; }}
        .nav-buttons button:disabled {{ opacity: 0.5; cursor: not-allowed; }}
        .progress {{
            font-size: 12px;
            color: #888;
            text-align: center;
        }}
        .labels {{
            max-height: 200px;
            overflow-y: auto;
        }}
        .label-item {{
            padding: 6px 10px;
            font-size: 13px;
            border-radius: 4px;
            cursor: pointer;
            margin-bottom: 4px;
        }}
        .label-item:hover {{ background: #0f3460; }}
        .label-item.current {{ background: #e94560; }}
        .variables {{
            font-size: 12px;
            font-family: monospace;
        }}
        .var-item {{
            display: flex;
            justify-content: space-between;
            padding: 4px 0;
            border-bottom: 1px solid #0f3460;
        }}
        .var-name {{ color: #4ade80; }}
        .var-value {{ color: #fbbf24; }}
        .info-row {{
            display: flex;
            justify-content: space-between;
            font-size: 12px;
            padding: 4px 0;
        }}
        .info-label {{ color: #888; }}
        .info-value {{ color: #4ade80; font-family: monospace; }}
    </style>
</head>
<body>
    <header>
        <h1>🎭 ivy Preview</h1>
        <span class="status" id="status">Connecting...</span>
    </header>
    <main>
        <div class="preview-area">
            <div class="scene" id="scene">
                <div class="background" id="background"></div>
                <img class="character" id="character" style="display:none" />
                <span class="no-image" id="no-image">No scene loaded</span>
            </div>
            <div class="textbox" id="textbox">
                <div class="speaker" id="speaker"></div>
                <div class="text" id="text"></div>
                <div class="choices" id="choices"></div>
            </div>
        </div>
        <div class="sidebar">
            <div class="panel">
                <h2>Navigation</h2>
                <div class="nav-buttons">
                    <button id="prev">◀ Prev</button>
                    <button id="next">Next ▶</button>
                </div>
                <div class="progress" id="progress">0 / 0</div>
            </div>
            <div class="panel">
                <h2>Labels</h2>
                <div class="labels" id="labels"></div>
            </div>
            <div class="panel">
                <h2>Variables</h2>
                <div class="variables" id="variables">No variables</div>
            </div>
            <div class="panel">
                <h2>Scene Info</h2>
                <div id="info">
                    <div class="info-row">
                        <span class="info-label">Title:</span>
                        <span class="info-value" id="info-title">-</span>
                    </div>
                    <div class="info-row">
                        <span class="info-label">Background:</span>
                        <span class="info-value" id="info-bg">-</span>
                    </div>
                    <div class="info-row">
                        <span class="info-label">Character:</span>
                        <span class="info-value" id="info-char">-</span>
                    </div>
                    <div class="info-row">
                        <span class="info-label">Mode:</span>
                        <span class="info-value" id="info-mode">ADV</span>
                    </div>
                </div>
            </div>
        </div>
    </main>
    <script>
        let ws;
        let state = null;
        let commandIndex = 0;

        function connect() {{
            ws = new WebSocket('ws://localhost:{ws_port}');
            ws.onopen = () => {{
                document.getElementById('status').textContent = 'Connected';
                document.getElementById('status').className = 'status connected';
                ws.send(JSON.stringify({{ type: 'get_state' }}));
            }};
            ws.onclose = () => {{
                document.getElementById('status').textContent = 'Disconnected';
                document.getElementById('status').className = 'status';
                setTimeout(connect, 2000);
            }};
            ws.onmessage = (e) => {{
                const msg = JSON.parse(e.data);
                if (msg.type === 'state') {{
                    state = msg;
                    commandIndex = msg.command_index;
                    render();
                }} else if (msg.type === 'reload') {{
                    ws.send(JSON.stringify({{ type: 'get_state' }}));
                }} else if (msg.type === 'error') {{
                    console.error('Server error:', msg.message);
                }}
            }};
        }}

        function render() {{
            if (!state) return;

            // Background
            const bg = document.getElementById('background');
            const noImage = document.getElementById('no-image');
            if (state.background) {{
                bg.style.backgroundImage = `url(/asset/${{state.background}})`;
                noImage.style.display = 'none';
            }} else {{
                bg.style.backgroundImage = '';
                noImage.style.display = state.character ? 'none' : 'block';
            }}

            // Character
            const char = document.getElementById('character');
            if (state.character) {{
                char.src = `/asset/${{state.character}}`;
                char.style.display = 'block';
            }} else {{
                char.style.display = 'none';
            }}

            // Textbox
            const textbox = document.getElementById('textbox');
            textbox.className = state.nvl_mode ? 'textbox nvl' : 'textbox';

            // Speaker
            const speaker = document.getElementById('speaker');
            speaker.textContent = state.speaker || '';
            speaker.style.display = state.speaker ? 'block' : 'none';

            // Text
            document.getElementById('text').textContent = state.text || '';

            // Choices
            const choicesEl = document.getElementById('choices');
            choicesEl.innerHTML = '';
            if (state.choices && state.choices.length > 0) {{
                state.choices.forEach((c, i) => {{
                    const div = document.createElement('div');
                    div.className = 'choice';
                    div.textContent = c.label;
                    div.onclick = () => jumpToLabel(c.jump);
                    choicesEl.appendChild(div);
                }});
            }}

            // Progress
            document.getElementById('progress').textContent =
                `${{state.command_index + 1}} / ${{state.total_commands}}`;

            // Labels
            const labelsEl = document.getElementById('labels');
            labelsEl.innerHTML = '';
            state.labels.forEach(label => {{
                const div = document.createElement('div');
                div.className = 'label-item' + (label === state.current_label ? ' current' : '');
                div.textContent = label;
                div.onclick = () => jumpToLabel(label);
                labelsEl.appendChild(div);
            }});

            // Variables
            const varsEl = document.getElementById('variables');
            const varKeys = Object.keys(state.variables || {{}});
            if (varKeys.length > 0) {{
                varsEl.innerHTML = varKeys.map(k =>
                    `<div class="var-item"><span class="var-name">${{k}}</span><span class="var-value">${{state.variables[k]}}</span></div>`
                ).join('');
            }} else {{
                varsEl.textContent = 'No variables';
            }}

            // Info
            document.getElementById('info-title').textContent = state.title || '-';
            document.getElementById('info-bg').textContent = state.background ? state.background.split('/').pop() : '-';
            document.getElementById('info-char').textContent = state.character ? state.character.split('/').pop() : '-';
            document.getElementById('info-mode').textContent = state.nvl_mode ? 'NVL' : 'ADV';

            // Nav buttons
            document.getElementById('prev').disabled = commandIndex <= 0;
            document.getElementById('next').disabled = commandIndex >= state.total_commands - 1;
        }}

        function navigate(delta) {{
            commandIndex = Math.max(0, Math.min(state.total_commands - 1, commandIndex + delta));
            ws.send(JSON.stringify({{ type: 'goto', index: commandIndex }}));
        }}

        function jumpToLabel(label) {{
            if (label) {{
                ws.send(JSON.stringify({{ type: 'jump', label }}));
            }}
        }}

        document.getElementById('prev').onclick = () => navigate(-1);
        document.getElementById('next').onclick = () => navigate(1);

        document.addEventListener('keydown', (e) => {{
            if (e.key === 'ArrowLeft') navigate(-1);
            if (e.key === 'ArrowRight') navigate(1);
            if (e.key === ' ' || e.key === 'Enter') navigate(1);
        }});

        connect();
    </script>
</body>
</html>
"#,
        ws_port = ws_port
    )
}

fn handle_http(mut stream: TcpStream, http_port: u16, ws_port: u16, scenario_dir: &Path) {
    let mut buffer = [0; 4096];
    if stream.read(&mut buffer).is_err() {
        return;
    }

    let request = String::from_utf8_lossy(&buffer);
    let path = request
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .unwrap_or("/");

    let (status, content_type, body) = if path == "/" || path == "/index.html" {
        (
            "200 OK",
            "text/html",
            generate_html(http_port, ws_port).into_bytes(),
        )
    } else if path.starts_with("/asset/") {
        let asset_path = path.strip_prefix("/asset/").unwrap_or("");
        let full_path = scenario_dir.join(asset_path);
        if full_path.exists() {
            let ext = full_path.extension().and_then(|e| e.to_str()).unwrap_or("");
            let mime = match ext {
                "png" => "image/png",
                "jpg" | "jpeg" => "image/jpeg",
                "gif" => "image/gif",
                "webp" => "image/webp",
                "svg" => "image/svg+xml",
                "ogg" => "audio/ogg",
                "mp3" => "audio/mpeg",
                "wav" => "audio/wav",
                _ => "application/octet-stream",
            };
            match fs::read(&full_path) {
                Ok(data) => ("200 OK", mime, data),
                Err(_) => (
                    "500 Internal Server Error",
                    "text/plain",
                    b"Error reading file".to_vec(),
                ),
            }
        } else {
            ("404 Not Found", "text/plain", b"Asset not found".to_vec())
        }
    } else {
        ("404 Not Found", "text/plain", b"Not found".to_vec())
    };

    let response = format!(
        "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\n\r\n",
        status,
        content_type,
        body.len()
    );

    let _ = stream.write_all(response.as_bytes());
    let _ = stream.write_all(&body);
}

/// Helper to send an error message to the WebSocket client.
fn send_ws_error(
    websocket: &mut tungstenite::WebSocket<TcpStream>,
    message: &str,
) {
    let response = WsMessage::Error {
        message: message.to_string(),
    };
    if let Ok(json) = serde_json::to_string(&response) {
        let _ = websocket.write(Message::Text(json));
    }
}

/// Helper to send state to the WebSocket client. Returns false on lock error.
fn send_state(
    websocket: &mut tungstenite::WebSocket<TcpStream>,
    scenario: &Arc<Mutex<Scenario>>,
    state: &Arc<Mutex<(usize, HashMap<String, String>)>>,
) -> bool {
    let scn = match scenario.lock() {
        Ok(s) => s,
        Err(e) => {
            send_ws_error(websocket, &format!("Scenario lock poisoned: {}", e));
            return false;
        }
    };
    let st = match state.lock() {
        Ok(s) => s,
        Err(e) => {
            send_ws_error(websocket, &format!("State lock poisoned: {}", e));
            return false;
        }
    };
    let preview = build_preview_state(&scn, st.0, &st.1);
    let response = WsMessage::State(Box::new(preview));
    if let Ok(json) = serde_json::to_string(&response) {
        let _ = websocket.write(Message::Text(json));
    }
    true
}

fn handle_websocket(
    stream: TcpStream,
    scenario: Arc<Mutex<Scenario>>,
    state: Arc<Mutex<(usize, HashMap<String, String>)>>,
    _tx: Sender<()>,
) {
    let mut websocket = match accept(stream) {
        Ok(ws) => ws,
        Err(_) => return,
    };

    loop {
        let msg = match websocket.read() {
            Ok(m) => m,
            Err(_) => break,
        };

        if msg.is_close() {
            break;
        }

        if let Message::Text(text) = msg
            && let Ok(json) = serde_json::from_str::<serde_json::Value>(&text)
        {
            let msg_type = json.get("type").and_then(|t| t.as_str()).unwrap_or("");

            match msg_type {
                "get_state" => {
                    send_state(&mut websocket, &scenario, &state);
                }
                "goto" => {
                    if let Some(idx) = json.get("index").and_then(|i| i.as_u64()) {
                        let update_ok = match state.lock() {
                            Ok(mut st) => {
                                st.0 = idx as usize;
                                true
                            }
                            Err(e) => {
                                send_ws_error(
                                    &mut websocket,
                                    &format!("State lock poisoned: {}", e),
                                );
                                false
                            }
                        };
                        if update_ok {
                            send_state(&mut websocket, &scenario, &state);
                        }
                    }
                }
                "jump" => {
                    if let Some(label) = json.get("label").and_then(|l| l.as_str()) {
                        let label_idx = match scenario.lock() {
                            Ok(scn) => scn
                                .script
                                .iter()
                                .position(|cmd| cmd.label.as_deref() == Some(label)),
                            Err(e) => {
                                send_ws_error(
                                    &mut websocket,
                                    &format!("Scenario lock poisoned: {}", e),
                                );
                                continue;
                            }
                        };
                        if let Some(idx) = label_idx {
                            let update_ok = match state.lock() {
                                Ok(mut st) => {
                                    st.0 = idx;
                                    true
                                }
                                Err(e) => {
                                    send_ws_error(
                                        &mut websocket,
                                        &format!("State lock poisoned: {}", e),
                                    );
                                    false
                                }
                            };
                            if update_ok {
                                send_state(&mut websocket, &scenario, &state);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return ExitCode::from(1);
    }

    let mut port: u16 = 3000;
    let mut target: Option<&str> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                print_usage();
                return ExitCode::from(0);
            }
            "--port" => {
                if i + 1 < args.len() {
                    port = args[i + 1].parse().unwrap_or(3000);
                    i += 1;
                }
            }
            arg if !arg.starts_with('-') => {
                target = Some(arg);
            }
            arg => {
                eprintln!("Unknown option: {}", arg);
                print_usage();
                return ExitCode::from(1);
            }
        }
        i += 1;
    }

    let target = match target {
        Some(t) => t,
        None => {
            eprintln!("No scenario file specified");
            print_usage();
            return ExitCode::from(1);
        }
    };

    let path = Path::new(target);
    if !path.is_file() {
        eprintln!("Error: {} is not a file", target);
        return ExitCode::from(1);
    }

    let scenario_dir = path.parent().unwrap_or(Path::new(".")).to_path_buf();

    // Load initial scenario
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return ExitCode::from(1);
        }
    };

    let initial_scenario = match parse_scenario(&content) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Parse error: {}", e);
            return ExitCode::from(1);
        }
    };

    let scenario = Arc::new(Mutex::new(initial_scenario));
    let state = Arc::new(Mutex::new((0usize, HashMap::<String, String>::new())));
    let (reload_tx, _reload_rx) = channel::<()>();

    // File watcher
    let watch_path = path.to_path_buf();
    let scenario_clone = Arc::clone(&scenario);
    let state_clone = Arc::clone(&state);
    thread::spawn(move || {
        let (tx, rx) = channel();
        let mut watcher = match RecommendedWatcher::new(tx, Config::default()) {
            Ok(w) => w,
            Err(_) => return,
        };

        if watcher
            .watch(&watch_path, RecursiveMode::NonRecursive)
            .is_err()
        {
            return;
        }

        loop {
            match rx.recv_timeout(Duration::from_millis(500)) {
                Ok(Ok(event)) => {
                    if event.paths.iter().any(|p| p == &watch_path) {
                        thread::sleep(Duration::from_millis(100)); // Debounce
                        if let Ok(content) = fs::read_to_string(&watch_path)
                            && let Ok(new_scenario) = parse_scenario(&content)
                        {
                            match scenario_clone.lock() {
                                Ok(mut scn) => {
                                    *scn = new_scenario;
                                }
                                Err(e) => {
                                    eprintln!(
                                        "Warning: Scenario lock poisoned during reload: {}",
                                        e
                                    );
                                    continue;
                                }
                            }

                            // Reset to beginning on reload
                            match state_clone.lock() {
                                Ok(mut st) => {
                                    st.0 = 0;
                                }
                                Err(e) => {
                                    eprintln!(
                                        "Warning: State lock poisoned during reload: {}",
                                        e
                                    );
                                }
                            }

                            eprintln!("Scenario reloaded: {}", watch_path.display());
                        }
                    }
                }
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => continue,
                Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
                _ => continue,
            }
        }
    });

    let ws_port = port + 1;

    // HTTP server
    let http_listener = match TcpListener::bind(format!("127.0.0.1:{}", port)) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Failed to bind HTTP port {}: {}", port, e);
            return ExitCode::from(1);
        }
    };

    let scenario_dir_http = scenario_dir.clone();
    thread::spawn(move || {
        for stream in http_listener.incoming().flatten() {
            handle_http(stream, port, ws_port, &scenario_dir_http);
        }
    });

    // WebSocket server
    let ws_listener = match TcpListener::bind(format!("127.0.0.1:{}", ws_port)) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Failed to bind WebSocket port {}: {}", ws_port, e);
            return ExitCode::from(1);
        }
    };

    eprintln!("Preview server started:");
    eprintln!("  HTTP:      http://127.0.0.1:{}", port);
    eprintln!("  WebSocket: ws://127.0.0.1:{}", ws_port);
    eprintln!();
    eprintln!("Watching: {}", path.display());
    eprintln!("Press Ctrl+C to stop");

    for stream in ws_listener.incoming().flatten() {
        let scenario = Arc::clone(&scenario);
        let state = Arc::clone(&state);
        let tx = reload_tx.clone();
        thread::spawn(move || {
            handle_websocket(stream, scenario, state, tx);
        });
    }

    ExitCode::from(0)
}
