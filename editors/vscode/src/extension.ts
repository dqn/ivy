import * as vscode from 'vscode';
import * as cp from 'child_process';
import * as path from 'path';
import * as fs from 'fs';

type ChildProcess = cp.ChildProcess;

let previewProcess: ChildProcess | undefined;
let previewPanel: vscode.WebviewPanel | undefined;
let outputChannel: vscode.OutputChannel;

// Common paths where ivy binaries might be installed
const COMMON_PATHS = [
    // Cargo install location
    process.env.HOME ? path.join(process.env.HOME, '.cargo', 'bin') : '',
    // Project target directory (for development)
    'target/release',
    'target/debug',
    // System paths
    '/usr/local/bin',
    '/usr/bin',
].filter(p => p !== '');

interface BinaryPaths {
    ivyPreview: string | null;
    ivyValidate: string | null;
}

function findBinary(name: string, configPath: string): string | null {
    // Check if config path is set and exists
    if (configPath && fs.existsSync(configPath)) {
        return configPath;
    }

    // Check if binary is in PATH
    try {
        const result = cp.spawnSync(process.platform === 'win32' ? 'where' : 'which', [name]);
        if (result.status === 0) {
            return name; // Binary found in PATH
        }
    } catch {
        // Ignore errors
    }

    // Check common paths
    for (const basePath of COMMON_PATHS) {
        const fullPath = path.join(basePath, process.platform === 'win32' ? `${name}.exe` : name);
        if (fs.existsSync(fullPath)) {
            return fullPath;
        }
    }

    // Check workspace folder
    const workspaceFolders = vscode.workspace.workspaceFolders;
    if (workspaceFolders) {
        for (const folder of workspaceFolders) {
            for (const subPath of ['target/release', 'target/debug']) {
                const fullPath = path.join(folder.uri.fsPath, subPath, process.platform === 'win32' ? `${name}.exe` : name);
                if (fs.existsSync(fullPath)) {
                    return fullPath;
                }
            }
        }
    }

    return null;
}

function detectBinaries(): BinaryPaths {
    const config = vscode.workspace.getConfiguration('ivy');
    return {
        ivyPreview: findBinary('ivy-preview', config.get<string>('ivyPreviewPath', '')),
        ivyValidate: findBinary('ivy-validate', config.get<string>('ivyValidatePath', '')),
    };
}

async function showInstallGuide(missingBinary: string) {
    const message = `${missingBinary} not found. Would you like to see installation instructions?`;
    const result = await vscode.window.showWarningMessage(message, 'Show Instructions', 'Configure Path', 'Dismiss');

    if (result === 'Show Instructions') {
        const panel = vscode.window.createWebviewPanel(
            'ivyInstall',
            'Install Ivy CLI Tools',
            vscode.ViewColumn.One,
            {}
        );
        panel.webview.html = getInstallGuideHtml();
    } else if (result === 'Configure Path') {
        vscode.commands.executeCommand('workbench.action.openSettings', 'ivy');
    }
}

function getInstallGuideHtml(): string {
    return `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Install Ivy CLI Tools</title>
    <style>
        body {
            font-family: var(--vscode-font-family);
            padding: 20px;
            line-height: 1.6;
        }
        h1 { color: var(--vscode-foreground); }
        h2 { color: var(--vscode-foreground); margin-top: 24px; }
        code {
            background: var(--vscode-textCodeBlock-background);
            padding: 2px 6px;
            border-radius: 3px;
        }
        pre {
            background: var(--vscode-textCodeBlock-background);
            padding: 12px;
            border-radius: 6px;
            overflow-x: auto;
        }
        .step {
            margin: 16px 0;
            padding: 12px;
            background: var(--vscode-editor-background);
            border-left: 3px solid var(--vscode-activityBarBadge-background);
        }
        .note {
            background: var(--vscode-inputValidation-infoBackground);
            border: 1px solid var(--vscode-inputValidation-infoBorder);
            padding: 12px;
            border-radius: 4px;
            margin: 16px 0;
        }
    </style>
</head>
<body>
    <h1>Install Ivy CLI Tools</h1>

    <p>The Ivy VSCode extension requires CLI tools for preview and validation features.</p>

    <h2>Option 1: Install from Source (Recommended)</h2>

    <div class="step">
        <strong>Step 1:</strong> Clone the ivy repository
        <pre>git clone https://github.com/dqn/ivy.git
cd ivy</pre>
    </div>

    <div class="step">
        <strong>Step 2:</strong> Build the CLI tools
        <pre>cargo build --release</pre>
    </div>

    <div class="step">
        <strong>Step 3:</strong> Add to PATH (choose one method)

        <p><strong>Method A:</strong> Install globally via cargo</p>
        <pre>cargo install --path . --bin ivy-preview
cargo install --path . --bin ivy-validate</pre>

        <p><strong>Method B:</strong> Add target/release to your PATH</p>
        <pre># Add to ~/.bashrc, ~/.zshrc, or equivalent
export PATH="$PATH:/path/to/ivy/target/release"</pre>

        <p><strong>Method C:</strong> Configure path in VSCode settings</p>
        <p>Set <code>ivy.ivyPreviewPath</code> and <code>ivy.ivyValidatePath</code> to the full binary paths.</p>
    </div>

    <h2>Option 2: Download Pre-built Binaries</h2>

    <div class="step">
        <strong>Step 1:</strong> Go to <a href="https://github.com/dqn/ivy/releases">GitHub Releases</a>
    </div>

    <div class="step">
        <strong>Step 2:</strong> Download the archive for your platform
        <ul>
            <li>Windows: <code>ivy-vX.X.X-x86_64-pc-windows-msvc.zip</code></li>
            <li>macOS Intel: <code>ivy-vX.X.X-x86_64-apple-darwin.tar.gz</code></li>
            <li>macOS Apple Silicon: <code>ivy-vX.X.X-aarch64-apple-darwin.tar.gz</code></li>
            <li>Linux: <code>ivy-vX.X.X-x86_64-unknown-linux-gnu.tar.gz</code></li>
        </ul>
    </div>

    <div class="step">
        <strong>Step 3:</strong> Extract and add to PATH, or configure paths in VSCode settings
    </div>

    <div class="note">
        <strong>Note:</strong> After installation, restart VSCode or run <strong>Developer: Reload Window</strong> for changes to take effect.
    </div>

    <h2>Verify Installation</h2>
    <pre>ivy-preview --help
ivy-validate --help</pre>

    <h2>VSCode Settings</h2>
    <p>You can also manually configure the binary paths in VSCode settings:</p>
    <pre>{
    "ivy.ivyPreviewPath": "/path/to/ivy-preview",
    "ivy.ivyValidatePath": "/path/to/ivy-validate"
}</pre>
</body>
</html>`;
}

export function activate(context: vscode.ExtensionContext) {
    outputChannel = vscode.window.createOutputChannel('Ivy');

    // Check for binaries on activation
    const binaries = detectBinaries();
    if (binaries.ivyPreview) {
        outputChannel.appendLine(`Found ivy-preview: ${binaries.ivyPreview}`);
    }
    if (binaries.ivyValidate) {
        outputChannel.appendLine(`Found ivy-validate: ${binaries.ivyValidate}`);
    }

    const previewCommand = vscode.commands.registerCommand('ivy.preview', async () => {
        const editor = vscode.window.activeTextEditor;
        if (!editor) {
            vscode.window.showErrorMessage('No active editor');
            return;
        }

        const document = editor.document;
        const filePath = document.uri.fsPath;

        if (!filePath.endsWith('.yaml') && !filePath.endsWith('.yml')) {
            vscode.window.showErrorMessage('Please open a YAML scenario file');
            return;
        }

        await startPreview(filePath, context);
    });

    const validateCommand = vscode.commands.registerCommand('ivy.validate', async () => {
        const editor = vscode.window.activeTextEditor;
        if (!editor) {
            vscode.window.showErrorMessage('No active editor');
            return;
        }

        const document = editor.document;
        const filePath = document.uri.fsPath;

        await runValidation(filePath);
    });

    const installGuideCommand = vscode.commands.registerCommand('ivy.showInstallGuide', () => {
        const panel = vscode.window.createWebviewPanel(
            'ivyInstall',
            'Install Ivy CLI Tools',
            vscode.ViewColumn.One,
            {}
        );
        panel.webview.html = getInstallGuideHtml();
    });

    context.subscriptions.push(previewCommand, validateCommand, installGuideCommand, outputChannel);

    // Clean up on deactivation
    context.subscriptions.push({
        dispose: () => {
            stopPreview();
        }
    });
}

async function startPreview(filePath: string, _context: vscode.ExtensionContext) {
    const config = vscode.workspace.getConfiguration('ivy');
    const port = config.get<number>('previewPort', 3000);
    const ivyPreviewPath = findBinary('ivy-preview', config.get<string>('ivyPreviewPath', ''));

    if (!ivyPreviewPath) {
        await showInstallGuide('ivy-preview');
        return;
    }

    // Stop existing preview if running
    stopPreview();

    outputChannel.appendLine(`Starting preview server for: ${filePath}`);
    outputChannel.appendLine(`Using port: ${port}`);

    // Start the preview server
    previewProcess = cp.spawn(ivyPreviewPath, ['--port', port.toString(), filePath], {
        cwd: path.dirname(filePath)
    });

    previewProcess.stdout?.on('data', (data: unknown) => {
        outputChannel.appendLine(String(data));
    });

    previewProcess.stderr?.on('data', (data: unknown) => {
        outputChannel.appendLine(String(data));
    });

    previewProcess.on('error', (err: Error) => {
        vscode.window.showErrorMessage(`Failed to start ivy-preview: ${err.message}. Make sure ivy-preview is installed and in your PATH.`);
        outputChannel.appendLine(`Error: ${err.message}`);
    });

    previewProcess.on('close', (_code: number | null) => {
        outputChannel.appendLine('Preview server exited');
        previewProcess = undefined;
    });

    // Wait a bit for the server to start
    await new Promise(resolve => setTimeout(resolve, 500));

    // Create or show the webview panel
    if (previewPanel) {
        previewPanel.reveal(vscode.ViewColumn.Beside);
    } else {
        previewPanel = vscode.window.createWebviewPanel(
            'ivyPreview',
            'Ivy Preview',
            vscode.ViewColumn.Beside,
            {
                enableScripts: true,
                retainContextWhenHidden: true
            }
        );

        previewPanel.onDidDispose(() => {
            previewPanel = undefined;
            stopPreview();
        });
    }

    previewPanel.webview.html = getWebviewContent(port);
}

function stopPreview() {
    if (previewProcess) {
        previewProcess.kill();
        previewProcess = undefined;
        outputChannel.appendLine('Preview server stopped');
    }
}

async function runValidation(filePath: string) {
    const config = vscode.workspace.getConfiguration('ivy');
    const ivyValidatePath = findBinary('ivy-validate', config.get<string>('ivyValidatePath', ''));

    if (!ivyValidatePath) {
        await showInstallGuide('ivy-validate');
        return;
    }

    outputChannel.appendLine(`Validating: ${filePath}`);
    outputChannel.show();

    const validateProcess = cp.spawn(ivyValidatePath, ['--json', filePath], {
        cwd: path.dirname(filePath)
    });

    let output = '';

    validateProcess.stdout?.on('data', (data: unknown) => {
        output += String(data);
    });

    validateProcess.stderr?.on('data', (data: unknown) => {
        outputChannel.appendLine(String(data));
    });

    validateProcess.on('close', (_code: number | null) => {
        try {
            const result = JSON.parse(output);
            if (result.success) {
                vscode.window.showInformationMessage(`Validation passed! ${result.total_warnings} warning(s)`);
            } else {
                vscode.window.showErrorMessage(`Validation failed: ${result.total_errors} error(s), ${result.total_warnings} warning(s)`);
            }

            // Show issues in output
            for (const fileResult of result.results) {
                for (const issue of fileResult.issues) {
                    const prefix = issue.severity === 'error' ? '❌' : '⚠️';
                    outputChannel.appendLine(`${prefix} ${issue.message}`);
                }
            }
        } catch {
            outputChannel.appendLine('Failed to parse validation output');
        }
    });

    validateProcess.on('error', (err: Error) => {
        vscode.window.showErrorMessage(`Failed to run ivy-validate: ${err.message}`);
    });
}

function getWebviewContent(port: number): string {
    return `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Ivy Preview</title>
    <style>
        body, html {
            margin: 0;
            padding: 0;
            width: 100%;
            height: 100%;
            overflow: hidden;
        }
        iframe {
            width: 100%;
            height: 100%;
            border: none;
        }
        .error {
            display: flex;
            align-items: center;
            justify-content: center;
            height: 100%;
            color: #f44336;
            font-family: system-ui, sans-serif;
            text-align: center;
            padding: 20px;
        }
    </style>
</head>
<body>
    <iframe id="preview" src="http://127.0.0.1:${port}"></iframe>
    <script>
        const iframe = document.getElementById('preview');
        iframe.onerror = () => {
            document.body.innerHTML = '<div class="error">Failed to connect to preview server.<br>Make sure ivy-preview is running.</div>';
        };
    </script>
</body>
</html>`;
}

export function deactivate() {
    stopPreview();
}
