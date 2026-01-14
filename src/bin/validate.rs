//! CLI tool for validating ivy scenarios.
//!
//! Usage:
//!   ivy-validate <scenario.yaml>
//!   ivy-validate --all <directory>

use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

use ivy::scenario::{parse_scenario, validate_scenario, detect_circular_paths, Severity};

fn print_usage() {
    eprintln!("ivy-validate - Validate ivy scenario files");
    eprintln!();
    eprintln!("Usage:");
    eprintln!("  ivy-validate <scenario.yaml>     Validate a single scenario file");
    eprintln!("  ivy-validate --all <directory>   Validate all .yaml files in directory");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  -h, --help    Show this help message");
    eprintln!("  --all         Validate all YAML files in the specified directory");
    eprintln!("  --cycles      Also detect circular jump paths");
}

fn validate_file(path: &Path, check_cycles: bool) -> (usize, usize) {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("  Error reading file: {}", e);
            return (1, 0);
        }
    };

    let scenario = match parse_scenario(&content) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("  Parse error: {}", e);
            return (1, 0);
        }
    };

    let result = validate_scenario(&scenario);
    let mut errors = 0;
    let mut warnings = 0;

    for issue in &result.issues {
        let severity_str = match issue.severity {
            Severity::Error => {
                errors += 1;
                "ERROR"
            }
            Severity::Warning => {
                warnings += 1;
                "WARNING"
            }
        };

        let location = match issue.command_index {
            Some(idx) => format!(" (command {})", idx + 1),
            None => String::new(),
        };

        eprintln!("  {}{}: {}", severity_str, location, issue.message);
    }

    if check_cycles {
        let cycles = detect_circular_paths(&scenario);
        for cycle in &cycles {
            eprintln!(
                "  WARNING: Circular path detected: {} -> {}",
                cycle.join(" -> "),
                cycle.first().unwrap_or(&String::new())
            );
            warnings += 1;
        }
    }

    (errors, warnings)
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return ExitCode::from(1);
    }

    let mut check_cycles = false;
    let mut all_mode = false;
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
    let mut total_errors = 0;
    let mut total_warnings = 0;
    let mut files_checked = 0;

    if all_mode {
        if !path.is_dir() {
            eprintln!("Error: {} is not a directory", target);
            return ExitCode::from(1);
        }

        let entries = match fs::read_dir(path) {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Error reading directory: {}", e);
                return ExitCode::from(1);
            }
        };

        for entry in entries.flatten() {
            let file_path = entry.path();
            if let Some(ext) = file_path.extension() {
                if ext == "yaml" || ext == "yml" {
                    eprintln!("Validating: {}", file_path.display());
                    let (errors, warnings) = validate_file(&file_path, check_cycles);
                    total_errors += errors;
                    total_warnings += warnings;
                    files_checked += 1;
                }
            }
        }
    } else {
        if !path.is_file() {
            eprintln!("Error: {} is not a file", target);
            return ExitCode::from(1);
        }

        eprintln!("Validating: {}", path.display());
        let (errors, warnings) = validate_file(path, check_cycles);
        total_errors = errors;
        total_warnings = warnings;
        files_checked = 1;
    }

    eprintln!();
    eprintln!(
        "Checked {} file(s): {} error(s), {} warning(s)",
        files_checked, total_errors, total_warnings
    );

    if total_errors > 0 {
        ExitCode::from(1)
    } else {
        ExitCode::from(0)
    }
}
