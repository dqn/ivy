//! CLI tool for validating ivy scenarios.
//!
//! Usage:
//!   ivy-validate <scenario.yaml>
//!   ivy-validate --all <directory>
//!   ivy-validate --watch <directory>
//!   ivy-validate --json <scenario.yaml>

use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::mpsc::channel;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Duration;

use ivy::scenario::{detect_circular_paths, parse_scenario, validate_scenario, Severity};
#[cfg(not(target_arch = "wasm32"))]
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use serde::Serialize;

// ANSI color codes
const RED: &str = "\x1b[31m";
const YELLOW: &str = "\x1b[33m";
const GREEN: &str = "\x1b[32m";
const CYAN: &str = "\x1b[36m";
const BOLD: &str = "\x1b[1m";
const RESET: &str = "\x1b[0m";

/// JSON output structure for a single issue.
#[derive(Serialize)]
struct JsonIssue {
    severity: String,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    command_index: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
}

/// JSON output structure for a file's validation result.
#[derive(Serialize)]
struct JsonFileResult {
    file: String,
    errors: usize,
    warnings: usize,
    issues: Vec<JsonIssue>,
}

/// JSON output structure for the overall validation result.
#[derive(Serialize)]
struct JsonOutput {
    success: bool,
    files_checked: usize,
    total_errors: usize,
    total_warnings: usize,
    results: Vec<JsonFileResult>,
}

fn print_usage() {
    eprintln!("ivy-validate - Validate ivy scenario files");
    eprintln!();
    eprintln!("Usage:");
    eprintln!("  ivy-validate <scenario.yaml>     Validate a single scenario file");
    eprintln!("  ivy-validate --all <directory>   Validate all .yaml files in directory");
    #[cfg(not(target_arch = "wasm32"))]
    eprintln!("  ivy-validate --watch <directory> Watch directory and validate on changes");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  -h, --help    Show this help message");
    eprintln!("  --all         Validate all YAML files in the specified directory");
    #[cfg(not(target_arch = "wasm32"))]
    eprintln!("  --watch       Watch for file changes and re-validate automatically");
    eprintln!("  --cycles      Also detect circular jump paths");
    eprintln!("  --no-color    Disable colored output");
    eprintln!("  --json        Output results in JSON format (for CI/tooling integration)");
    eprintln!("  --quiet, -q   Only output errors (suppress warnings and info)");
}

fn validate_file(path: &Path, check_cycles: bool, use_color: bool) -> (usize, usize) {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            if use_color {
                eprintln!("  {}{}Error{} reading file: {}", BOLD, RED, RESET, e);
            } else {
                eprintln!("  Error reading file: {}", e);
            }
            return (1, 0);
        }
    };

    let scenario = match parse_scenario(&content) {
        Ok(s) => s,
        Err(e) => {
            if use_color {
                eprintln!("  {}{}Parse error{}: {}", BOLD, RED, RESET, e);
            } else {
                eprintln!("  Parse error: {}", e);
            }
            return (1, 0);
        }
    };

    let result = validate_scenario(&scenario);
    let mut errors = 0;
    let mut warnings = 0;

    for issue in &result.issues {
        let location = match issue.command_index {
            Some(idx) => format!(" (command {})", idx + 1),
            None => String::new(),
        };

        match issue.severity {
            Severity::Error => {
                errors += 1;
                if use_color {
                    eprintln!(
                        "  {}{}ERROR{}{}: {}",
                        BOLD, RED, RESET, location, issue.message
                    );
                } else {
                    eprintln!("  ERROR{}: {}", location, issue.message);
                }
            }
            Severity::Warning => {
                warnings += 1;
                if use_color {
                    eprintln!(
                        "  {}{}WARNING{}{}: {}",
                        BOLD, YELLOW, RESET, location, issue.message
                    );
                } else {
                    eprintln!("  WARNING{}: {}", location, issue.message);
                }
            }
        }
    }

    if check_cycles {
        let cycles = detect_circular_paths(&scenario);
        for cycle in &cycles {
            if use_color {
                eprintln!(
                    "  {}{}WARNING{}: Circular path detected: {} -> {}",
                    BOLD,
                    YELLOW,
                    RESET,
                    cycle.join(" -> "),
                    cycle.first().unwrap_or(&String::new())
                );
            } else {
                eprintln!(
                    "  WARNING: Circular path detected: {} -> {}",
                    cycle.join(" -> "),
                    cycle.first().unwrap_or(&String::new())
                );
            }
            warnings += 1;
        }
    }

    (errors, warnings)
}

/// Validate a file and return JSON-compatible result.
fn validate_file_json(path: &Path, check_cycles: bool) -> JsonFileResult {
    let mut issues = Vec::new();
    let mut errors = 0;
    let mut warnings = 0;

    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            issues.push(JsonIssue {
                severity: "error".to_string(),
                message: format!("Error reading file: {}", e),
                command_index: None,
                label: None,
            });
            return JsonFileResult {
                file: path.display().to_string(),
                errors: 1,
                warnings: 0,
                issues,
            };
        }
    };

    let scenario = match parse_scenario(&content) {
        Ok(s) => s,
        Err(e) => {
            issues.push(JsonIssue {
                severity: "error".to_string(),
                message: format!("Parse error: {}", e),
                command_index: None,
                label: None,
            });
            return JsonFileResult {
                file: path.display().to_string(),
                errors: 1,
                warnings: 0,
                issues,
            };
        }
    };

    let result = validate_scenario(&scenario);

    for issue in &result.issues {
        let severity = match issue.severity {
            Severity::Error => {
                errors += 1;
                "error"
            }
            Severity::Warning => {
                warnings += 1;
                "warning"
            }
        };

        issues.push(JsonIssue {
            severity: severity.to_string(),
            message: issue.message.clone(),
            command_index: issue.command_index,
            label: issue.label.clone(),
        });
    }

    if check_cycles {
        let cycles = detect_circular_paths(&scenario);
        for cycle in &cycles {
            warnings += 1;
            issues.push(JsonIssue {
                severity: "warning".to_string(),
                message: format!(
                    "Circular path detected: {} -> {}",
                    cycle.join(" -> "),
                    cycle.first().unwrap_or(&String::new())
                ),
                command_index: None,
                label: cycle.first().cloned(),
            });
        }
    }

    JsonFileResult {
        file: path.display().to_string(),
        errors,
        warnings,
        issues,
    }
}

fn validate_directory(path: &Path, check_cycles: bool, use_color: bool) -> (usize, usize, usize) {
    let entries = match fs::read_dir(path) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Error reading directory: {}", e);
            return (0, 0, 0);
        }
    };

    let mut total_errors = 0;
    let mut total_warnings = 0;
    let mut files_checked = 0;

    for entry in entries.flatten() {
        let file_path = entry.path();
        if let Some(ext) = file_path.extension() {
            if ext == "yaml" || ext == "yml" {
                if use_color {
                    eprintln!("{}Validating:{} {}", CYAN, RESET, file_path.display());
                } else {
                    eprintln!("Validating: {}", file_path.display());
                }
                let (errors, warnings) = validate_file(&file_path, check_cycles, use_color);
                total_errors += errors;
                total_warnings += warnings;
                files_checked += 1;
            }
        }
    }

    (total_errors, total_warnings, files_checked)
}

#[cfg(not(target_arch = "wasm32"))]
fn watch_directory(path: &Path, check_cycles: bool, use_color: bool) -> Result<(), notify::Error> {
    let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    watcher.watch(path, RecursiveMode::Recursive)?;

    if use_color {
        eprintln!(
            "\n{}{}Watching{} {} for changes (press Ctrl+C to stop)...\n",
            BOLD,
            GREEN,
            RESET,
            path.display()
        );
    } else {
        eprintln!(
            "\nWatching {} for changes (press Ctrl+C to stop)...\n",
            path.display()
        );
    }

    // Initial validation
    let (errors, warnings, files) = validate_directory(path, check_cycles, use_color);
    print_summary(errors, warnings, files, use_color);

    loop {
        match rx.recv_timeout(Duration::from_millis(500)) {
            Ok(result) => {
                if let Ok(event) = result {
                    // Check if any YAML files were modified
                    let yaml_changed = event.paths.iter().any(|p| {
                        p.extension()
                            .is_some_and(|ext| ext == "yaml" || ext == "yml")
                    });

                    if yaml_changed {
                        if use_color {
                            eprintln!("\n{}--- File changed, re-validating ---{}", CYAN, RESET);
                        } else {
                            eprintln!("\n--- File changed, re-validating ---");
                        }
                        let (errors, warnings, files) =
                            validate_directory(path, check_cycles, use_color);
                        print_summary(errors, warnings, files, use_color);
                    }
                }
            }
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => continue,
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
        }
    }

    Ok(())
}

fn print_summary(errors: usize, warnings: usize, files: usize, use_color: bool) {
    eprintln!();
    if use_color {
        let status = if errors > 0 {
            format!("{}{}FAIL{}", BOLD, RED, RESET)
        } else {
            format!("{}{}OK{}", BOLD, GREEN, RESET)
        };
        eprintln!(
            "[{}] Checked {} file(s): {} error(s), {} warning(s)",
            status, files, errors, warnings
        );
    } else {
        eprintln!(
            "Checked {} file(s): {} error(s), {} warning(s)",
            files, errors, warnings
        );
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return ExitCode::from(1);
    }

    let mut check_cycles = false;
    let mut all_mode = false;
    #[cfg(not(target_arch = "wasm32"))]
    let mut watch_mode = false;
    let mut use_color = true;
    let mut json_mode = false;
    let mut quiet_mode = false;
    let mut target: Option<&str> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                print_usage();
                return ExitCode::from(0);
            }
            "--cycles" => {
                check_cycles = true;
            }
            "--all" => {
                all_mode = true;
            }
            #[cfg(not(target_arch = "wasm32"))]
            "--watch" => {
                watch_mode = true;
            }
            "--no-color" => {
                use_color = false;
            }
            "--json" => {
                json_mode = true;
            }
            "-q" | "--quiet" => {
                quiet_mode = true;
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
            eprintln!("No target specified");
            print_usage();
            return ExitCode::from(1);
        }
    };

    let path = Path::new(target);

    // JSON mode
    if json_mode {
        let mut results = Vec::new();
        let mut total_errors = 0;
        let mut total_warnings = 0;

        if all_mode || path.is_dir() {
            if !path.is_dir() {
                eprintln!("Error: {} is not a directory", target);
                return ExitCode::from(1);
            }

            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries.flatten() {
                    let file_path = entry.path();
                    if let Some(ext) = file_path.extension() {
                        if ext == "yaml" || ext == "yml" {
                            let result = validate_file_json(&file_path, check_cycles);
                            total_errors += result.errors;
                            total_warnings += result.warnings;
                            results.push(result);
                        }
                    }
                }
            }
        } else {
            if !path.is_file() {
                eprintln!("Error: {} is not a file", target);
                return ExitCode::from(1);
            }
            let result = validate_file_json(path, check_cycles);
            total_errors += result.errors;
            total_warnings += result.warnings;
            results.push(result);
        }

        let output = JsonOutput {
            success: total_errors == 0,
            files_checked: results.len(),
            total_errors,
            total_warnings,
            results,
        };

        if let Ok(json) = serde_json::to_string_pretty(&output) {
            println!("{}", json);
        }

        return if total_errors > 0 {
            ExitCode::from(1)
        } else {
            ExitCode::from(0)
        };
    }

    // Watch mode
    #[cfg(not(target_arch = "wasm32"))]
    if watch_mode {
        if !path.is_dir() {
            eprintln!("Error: --watch requires a directory");
            return ExitCode::from(1);
        }

        if let Err(e) = watch_directory(path, check_cycles, use_color) {
            eprintln!("Watch error: {}", e);
            return ExitCode::from(1);
        }
        return ExitCode::from(0);
    }

    let (total_errors, total_warnings, files_checked) = if all_mode {
        if !path.is_dir() {
            eprintln!("Error: {} is not a directory", target);
            return ExitCode::from(1);
        }
        validate_directory(path, check_cycles, use_color)
    } else {
        if !path.is_file() {
            eprintln!("Error: {} is not a file", target);
            return ExitCode::from(1);
        }

        if !quiet_mode {
            if use_color {
                eprintln!("{}Validating:{} {}", CYAN, RESET, path.display());
            } else {
                eprintln!("Validating: {}", path.display());
            }
        }
        let (errors, warnings) = validate_file(path, check_cycles, use_color);
        (errors, warnings, 1)
    };

    if !quiet_mode || total_errors > 0 {
        print_summary(total_errors, total_warnings, files_checked, use_color);
    }

    if total_errors > 0 {
        ExitCode::from(1)
    } else {
        ExitCode::from(0)
    }
}
