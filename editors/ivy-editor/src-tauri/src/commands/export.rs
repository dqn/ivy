use anyhow::Result;
use flate2::write::GzEncoder;
use flate2::Compression;
use image::ImageReader;
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

static EXPORT_CANCELLED: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportTarget {
    CurrentPlatform,
    Windows,
    Macos,
    Linux,
    Web,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageCompressionOptions {
    pub format: String,
    pub quality: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConversionOptions {
    pub format: String,
    pub bitrate: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageFormat {
    None,
    Zip,
    TarGz,
    AppBundle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportOptions {
    pub target: ExportTarget,
    pub output_dir: String,
    pub release_build: bool,
    pub optimize_assets: bool,
    pub image_compression: Option<ImageCompressionOptions>,
    pub audio_conversion: Option<AudioConversionOptions>,
    pub exclude_unused_assets: bool,
    pub package_format: PackageFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildEnvironment {
    pub has_rust: bool,
    pub rust_version: Option<String>,
    pub has_cargo: bool,
    pub has_wasm_pack: bool,
    pub current_platform: ExportTarget,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportStage {
    CheckingEnvironment,
    OptimizingAssets,
    Building,
    Packaging,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportProgress {
    pub stage: ExportStage,
    pub message: String,
    pub progress: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportResult {
    pub success: bool,
    pub output_path: Option<String>,
    pub error: Option<String>,
    pub warnings: Vec<String>,
}

fn check_command_exists(cmd: &str) -> bool {
    Command::new(cmd).arg("--version").output().is_ok()
}

fn get_rust_version() -> Option<String> {
    Command::new("rustc")
        .arg("--version")
        .output()
        .ok()
        .and_then(|output| {
            String::from_utf8(output.stdout)
                .ok()
                .map(|s| s.trim().to_string())
        })
}

fn get_current_platform() -> ExportTarget {
    #[cfg(target_os = "windows")]
    return ExportTarget::Windows;

    #[cfg(target_os = "macos")]
    return ExportTarget::Macos;

    #[cfg(target_os = "linux")]
    return ExportTarget::Linux;

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    return ExportTarget::Linux;
}

#[tauri::command]
pub fn check_build_environment() -> BuildEnvironment {
    let has_rust = check_command_exists("rustc");
    let has_cargo = check_command_exists("cargo");
    let has_wasm_pack = check_command_exists("wasm-pack");
    let rust_version = if has_rust { get_rust_version() } else { None };

    BuildEnvironment {
        has_rust,
        rust_version,
        has_cargo,
        has_wasm_pack,
        current_platform: get_current_platform(),
    }
}

fn emit_progress(app: &AppHandle, stage: ExportStage, message: &str, progress: u32) {
    let _ = app.emit(
        "export-progress",
        ExportProgress {
            stage,
            message: message.to_string(),
            progress,
        },
    );
}

fn copy_assets(
    project_path: &Path,
    output_dir: &Path,
    options: &ExportOptions,
    warnings: &mut Vec<String>,
) -> Result<()> {
    let assets_dir = project_path.join("assets");
    if !assets_dir.exists() {
        warnings.push("No assets directory found".to_string());
        return Ok(());
    }

    let output_assets = output_dir.join("assets");
    if output_assets.exists() {
        std::fs::remove_dir_all(&output_assets)?;
    }

    if options.optimize_assets {
        copy_and_optimize_assets(&assets_dir, &output_assets, options, warnings)?;
    } else {
        copy_dir_recursive(&assets_dir, &output_assets)?;
    }

    Ok(())
}

fn copy_and_optimize_assets(
    src: &Path,
    dst: &Path,
    options: &ExportOptions,
    warnings: &mut Vec<String>,
) -> Result<()> {
    std::fs::create_dir_all(dst)?;

    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = entry.file_name();

        if path.is_dir() {
            copy_and_optimize_assets(&path, &dst.join(&file_name), options, warnings)?;
        } else {
            let ext = path.extension().and_then(OsStr::to_str).unwrap_or("");
            let ext_lower = ext.to_lowercase();

            // Handle images
            if is_image_extension(&ext_lower) {
                if let Some(ref img_opts) = options.image_compression {
                    match optimize_image(&path, dst, img_opts) {
                        Ok(()) => continue,
                        Err(e) => {
                            warnings.push(format!(
                                "Failed to optimize {}: {}",
                                path.display(),
                                e
                            ));
                            // Fall back to copy
                        }
                    }
                }
            }

            // Handle audio
            if is_audio_extension(&ext_lower) {
                if let Some(ref audio_opts) = options.audio_conversion {
                    match convert_audio(&path, dst, audio_opts) {
                        Ok(()) => continue,
                        Err(e) => {
                            warnings.push(format!(
                                "Failed to convert audio {}: {}",
                                path.display(),
                                e
                            ));
                            // Fall back to copy
                        }
                    }
                }
            }

            // Default: just copy
            std::fs::copy(&path, dst.join(&file_name))?;
        }
    }

    Ok(())
}

fn is_image_extension(ext: &str) -> bool {
    matches!(ext, "png" | "jpg" | "jpeg" | "gif" | "bmp" | "tiff" | "webp")
}

fn is_audio_extension(ext: &str) -> bool {
    matches!(ext, "wav" | "mp3" | "ogg" | "flac")
}

fn optimize_image(
    src: &Path,
    dst_dir: &Path,
    options: &ImageCompressionOptions,
) -> Result<()> {
    let img = ImageReader::open(src)?.decode()?;
    let stem = src.file_stem().and_then(OsStr::to_str).unwrap_or("image");

    match options.format.as_str() {
        "webp" => {
            let output_path = dst_dir.join(format!("{}.webp", stem));
            let rgba = img.to_rgba8();
            let encoder = webp::Encoder::from_rgba(&rgba, rgba.width(), rgba.height());
            let webp_data = encoder.encode(options.quality as f32);
            std::fs::write(output_path, &*webp_data)?;
        }
        "jpeg" | "jpg" => {
            let output_path = dst_dir.join(format!("{}.jpg", stem));
            let rgb = img.to_rgb8();
            let mut output = std::fs::File::create(&output_path)?;
            let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(
                &mut output,
                options.quality as u8,
            );
            encoder.encode(
                rgb.as_raw(),
                rgb.width(),
                rgb.height(),
                image::ExtendedColorType::Rgb8,
            )?;
        }
        "png" => {
            let output_path = dst_dir.join(format!("{}.png", stem));
            img.save(&output_path)?;
        }
        _ => {
            return Err(anyhow::anyhow!("Unsupported image format: {}", options.format));
        }
    }

    Ok(())
}

fn convert_audio(
    src: &Path,
    dst_dir: &Path,
    options: &AudioConversionOptions,
) -> Result<()> {
    let stem = src.file_stem().and_then(OsStr::to_str).unwrap_or("audio");
    let ext = src.extension().and_then(OsStr::to_str).unwrap_or("");

    // If source is already in target format, just copy
    if ext.to_lowercase() == options.format {
        std::fs::copy(src, dst_dir.join(src.file_name().unwrap()))?;
        return Ok(());
    }

    // Use ffmpeg for audio conversion if available
    let output_ext = match options.format.as_str() {
        "ogg" => "ogg",
        "mp3" => "mp3",
        _ => return Err(anyhow::anyhow!("Unsupported audio format: {}", options.format)),
    };

    let output_path = dst_dir.join(format!("{}.{}", stem, output_ext));

    // Try ffmpeg
    let result = Command::new("ffmpeg")
        .args([
            "-i",
            src.to_str().unwrap(),
            "-b:a",
            &format!("{}k", options.bitrate),
            "-y",
            output_path.to_str().unwrap(),
        ])
        .output();

    match result {
        Ok(output) if output.status.success() => Ok(()),
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(anyhow::anyhow!("ffmpeg failed: {}", stderr))
        }
        Err(_) => {
            // ffmpeg not available, just copy the original
            std::fs::copy(src, dst_dir.join(src.file_name().unwrap()))?;
            Err(anyhow::anyhow!("ffmpeg not found, copied original file"))
        }
    }
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    std::fs::create_dir_all(dst)?;

    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = entry.file_name();
        let dest_path = dst.join(&file_name);

        if path.is_dir() {
            copy_dir_recursive(&path, &dest_path)?;
        } else {
            std::fs::copy(&path, &dest_path)?;
        }
    }

    Ok(())
}

fn build_native(
    app: &AppHandle,
    project_path: &Path,
    output_dir: &Path,
    options: &ExportOptions,
    warnings: &mut Vec<String>,
) -> Result<String> {
    emit_progress(
        app,
        ExportStage::Building,
        "Building native executable...",
        30,
    );

    // Find the ivy crate root (parent of project)
    let ivy_root = find_ivy_root(project_path)?;

    let mut cmd = Command::new("cargo");
    cmd.current_dir(&ivy_root).arg("build");

    if options.release_build {
        cmd.arg("--release");
    }

    let output = cmd.output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("Build failed:\n{}", stderr));
    }

    if !output.stderr.is_empty() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        for line in stderr.lines() {
            if line.contains("warning:") {
                warnings.push(line.to_string());
            }
        }
    }

    if options.optimize_assets {
        emit_progress(
            app,
            ExportStage::OptimizingAssets,
            "Optimizing assets...",
            50,
        );
    }

    emit_progress(app, ExportStage::Packaging, "Copying files...", 70);

    // Determine binary name and path
    let profile = if options.release_build {
        "release"
    } else {
        "debug"
    };

    #[cfg(target_os = "windows")]
    let binary_name = "ivy.exe";
    #[cfg(not(target_os = "windows"))]
    let binary_name = "ivy";

    let binary_path = ivy_root.join("target").join(profile).join(binary_name);

    if !binary_path.exists() {
        return Err(anyhow::anyhow!(
            "Binary not found at {}",
            binary_path.display()
        ));
    }

    // Create output directory
    std::fs::create_dir_all(output_dir)?;

    // Copy binary
    let output_binary = output_dir.join(binary_name);
    std::fs::copy(&binary_path, &output_binary)?;

    // Copy assets
    copy_assets(project_path, output_dir, options, warnings)?;

    // Copy scenarios
    let scenarios_dir = project_path.join("scenarios");
    if scenarios_dir.exists() {
        copy_dir_recursive(&scenarios_dir, &output_dir.join("scenarios"))?;
    }

    Ok(output_dir.to_string_lossy().to_string())
}

fn build_web(
    app: &AppHandle,
    project_path: &Path,
    output_dir: &Path,
    options: &ExportOptions,
    warnings: &mut Vec<String>,
) -> Result<String> {
    emit_progress(app, ExportStage::Building, "Building WASM...", 30);

    let ivy_root = find_ivy_root(project_path)?;

    let mut cmd = Command::new("wasm-pack");
    cmd.current_dir(&ivy_root)
        .arg("build")
        .arg("--target")
        .arg("web");

    if options.release_build {
        cmd.arg("--release");
    } else {
        cmd.arg("--dev");
    }

    let output = cmd.output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("WASM build failed:\n{}", stderr));
    }

    if options.optimize_assets {
        emit_progress(
            app,
            ExportStage::OptimizingAssets,
            "Optimizing assets...",
            50,
        );
    }

    emit_progress(app, ExportStage::Packaging, "Packaging web build...", 70);

    // Create output directory
    std::fs::create_dir_all(output_dir)?;

    // Copy WASM output
    let pkg_dir = ivy_root.join("pkg");
    if pkg_dir.exists() {
        copy_dir_recursive(&pkg_dir, &output_dir.join("pkg"))?;
    }

    // Copy assets
    copy_assets(project_path, output_dir, options, warnings)?;

    // Copy scenarios
    let scenarios_dir = project_path.join("scenarios");
    if scenarios_dir.exists() {
        copy_dir_recursive(&scenarios_dir, &output_dir.join("scenarios"))?;
    }

    // Create index.html
    let index_html = include_str!("../templates/web_index.html");
    std::fs::write(output_dir.join("index.html"), index_html)?;

    Ok(output_dir.to_string_lossy().to_string())
}

fn find_ivy_root(project_path: &Path) -> Result<std::path::PathBuf> {
    // Look for Cargo.toml with ivy package
    let mut current = project_path.to_path_buf();

    for _ in 0..10 {
        let cargo_toml = current.join("Cargo.toml");
        if cargo_toml.exists() {
            let content = std::fs::read_to_string(&cargo_toml)?;
            if content.contains("name = \"ivy\"") {
                return Ok(current);
            }
        }

        if let Some(parent) = current.parent() {
            current = parent.to_path_buf();
        } else {
            break;
        }
    }

    Err(anyhow::anyhow!(
        "Could not find ivy crate root. Make sure your project is inside the ivy repository."
    ))
}

#[tauri::command]
pub async fn start_export(
    app: AppHandle,
    project_path: String,
    options: ExportOptions,
) -> ExportResult {
    EXPORT_CANCELLED.store(false, Ordering::SeqCst);

    let app = Arc::new(app);
    let mut warnings = Vec::new();

    emit_progress(
        &app,
        ExportStage::CheckingEnvironment,
        "Checking build environment...",
        0,
    );

    let env = check_build_environment();

    // Validate environment
    match options.target {
        ExportTarget::Web => {
            if !env.has_wasm_pack {
                return ExportResult {
                    success: false,
                    output_path: None,
                    error: Some(
                        "wasm-pack is required for web builds. Install it with: cargo install wasm-pack"
                            .to_string(),
                    ),
                    warnings,
                };
            }
        }
        _ => {
            if !env.has_rust || !env.has_cargo {
                return ExportResult {
                    success: false,
                    output_path: None,
                    error: Some(
                        "Rust and Cargo are required for native builds. Install from: https://rustup.rs"
                            .to_string(),
                    ),
                    warnings,
                };
            }
        }
    }

    if EXPORT_CANCELLED.load(Ordering::SeqCst) {
        return ExportResult {
            success: false,
            output_path: None,
            error: Some("Export cancelled".to_string()),
            warnings,
        };
    }

    let project_path_ref = Path::new(&project_path);
    let output_dir = Path::new(&options.output_dir);

    // Determine build directory based on package format
    let (build_dir, needs_packaging) = match options.package_format {
        PackageFormat::None => (output_dir.to_path_buf(), false),
        _ => {
            // Build to a temporary directory, then package
            let temp_dir = output_dir.join("_build_temp");
            (temp_dir, true)
        }
    };

    // Extract project name from path
    let project_name = project_path_ref
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("ivy_game")
        .to_string();

    let result = match options.target {
        ExportTarget::Web => {
            build_web(&app, project_path_ref, &build_dir, &options, &mut warnings)
        }
        _ => build_native(&app, project_path_ref, &build_dir, &options, &mut warnings),
    };

    let final_result = match result {
        Ok(_) if needs_packaging => {
            // Package the build
            create_package(
                &app,
                &build_dir,
                output_dir,
                &project_name,
                &options.package_format,
            )
        }
        other => other,
    };

    match final_result {
        Ok(output_path) => {
            emit_progress(
                &app,
                ExportStage::Completed,
                "Export completed successfully!",
                100,
            );
            ExportResult {
                success: true,
                output_path: Some(output_path),
                error: None,
                warnings,
            }
        }
        Err(e) => {
            // Clean up temp directory if it exists
            if needs_packaging {
                let _ = std::fs::remove_dir_all(&build_dir);
            }
            emit_progress(&app, ExportStage::Failed, &e.to_string(), 100);
            ExportResult {
                success: false,
                output_path: None,
                error: Some(e.to_string()),
                warnings,
            }
        }
    }
}

#[tauri::command]
pub fn cancel_export() {
    EXPORT_CANCELLED.store(true, Ordering::SeqCst);
}

fn create_package(
    app: &AppHandle,
    build_dir: &Path,
    output_dir: &Path,
    project_name: &str,
    format: &PackageFormat,
) -> Result<String> {
    match format {
        PackageFormat::None => Ok(build_dir.to_string_lossy().to_string()),
        PackageFormat::Zip => {
            emit_progress(app, ExportStage::Packaging, "Creating ZIP archive...", 85);
            create_zip(build_dir, output_dir, project_name)
        }
        PackageFormat::TarGz => {
            emit_progress(
                app,
                ExportStage::Packaging,
                "Creating TAR.GZ archive...",
                85,
            );
            create_tar_gz(build_dir, output_dir, project_name)
        }
        PackageFormat::AppBundle => {
            emit_progress(
                app,
                ExportStage::Packaging,
                "Creating macOS app bundle...",
                85,
            );
            create_app_bundle(build_dir, output_dir, project_name)
        }
    }
}

fn create_zip(src_dir: &Path, output_dir: &Path, name: &str) -> Result<String> {
    let zip_path = output_dir.join(format!("{}.zip", name));
    let file = File::create(&zip_path)?;
    let mut zip = ZipWriter::new(file);
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    add_dir_to_zip(&mut zip, src_dir, src_dir, &options)?;
    zip.finish()?;

    // Clean up build directory
    std::fs::remove_dir_all(src_dir)?;

    Ok(zip_path.to_string_lossy().to_string())
}

fn add_dir_to_zip(
    zip: &mut ZipWriter<File>,
    base_dir: &Path,
    current_dir: &Path,
    options: &SimpleFileOptions,
) -> Result<()> {
    for entry in std::fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();
        let relative_path = path.strip_prefix(base_dir)?;
        let name = relative_path.to_string_lossy();

        if path.is_dir() {
            zip.add_directory(format!("{}/", name), *options)?;
            add_dir_to_zip(zip, base_dir, &path, options)?;
        } else {
            zip.start_file(name.to_string(), *options)?;
            let mut file = File::open(&path)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
        }
    }
    Ok(())
}

fn create_tar_gz(src_dir: &Path, output_dir: &Path, name: &str) -> Result<String> {
    let tar_gz_path = output_dir.join(format!("{}.tar.gz", name));
    let file = File::create(&tar_gz_path)?;
    let encoder = GzEncoder::new(file, Compression::default());
    let mut tar = tar::Builder::new(encoder);

    tar.append_dir_all(name, src_dir)?;
    tar.finish()?;

    // Clean up build directory
    std::fs::remove_dir_all(src_dir)?;

    Ok(tar_gz_path.to_string_lossy().to_string())
}

fn create_app_bundle(src_dir: &Path, output_dir: &Path, name: &str) -> Result<String> {
    let app_name = format!("{}.app", name);
    let app_path = output_dir.join(&app_name);

    // Create bundle structure
    let contents = app_path.join("Contents");
    let macos = contents.join("MacOS");
    let resources = contents.join("Resources");

    std::fs::create_dir_all(&macos)?;
    std::fs::create_dir_all(&resources)?;

    // Move executable to MacOS folder
    let binary_name = "ivy";
    let src_binary = src_dir.join(binary_name);
    if src_binary.exists() {
        std::fs::copy(&src_binary, macos.join(binary_name))?;
        // Make executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(macos.join(binary_name))?.permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(macos.join(binary_name), perms)?;
        }
    }

    // Copy assets and scenarios to Resources
    let assets = src_dir.join("assets");
    if assets.exists() {
        copy_dir_recursive(&assets, &resources.join("assets"))?;
    }

    let scenarios = src_dir.join("scenarios");
    if scenarios.exists() {
        copy_dir_recursive(&scenarios, &resources.join("scenarios"))?;
    }

    // Create Info.plist
    let info_plist = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>{}</string>
    <key>CFBundleIdentifier</key>
    <string>com.ivy.{}</string>
    <key>CFBundleName</key>
    <string>{}</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0.0</string>
    <key>CFBundleVersion</key>
    <string>1</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.15</string>
    <key>NSHighResolutionCapable</key>
    <true/>
</dict>
</plist>"#,
        binary_name,
        name.to_lowercase().replace(' ', "-"),
        name
    );

    std::fs::write(contents.join("Info.plist"), info_plist)?;

    // Clean up original build directory
    std::fs::remove_dir_all(src_dir)?;

    Ok(app_path.to_string_lossy().to_string())
}
