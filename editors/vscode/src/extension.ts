import * as vscode from 'vscode';
import * as cp from 'child_process';
import * as path from 'path';

type ChildProcess = cp.ChildProcess;

let previewProcess: ChildProcess | undefined;
let previewPanel: vscode.WebviewPanel | undefined;
let outputChannel: vscode.OutputChannel;

export function activate(context: vscode.ExtensionContext) {
    outputChannel = vscode.window.createOutputChannel('Ivy Preview');

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

    context.subscriptions.push(previewCommand, validateCommand, outputChannel);

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
    let ivyPreviewPath = config.get<string>('ivyPreviewPath', '');

    if (!ivyPreviewPath) {
        ivyPreviewPath = 'ivy-preview';
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
    outputChannel.appendLine(`Validating: ${filePath}`);
    outputChannel.show();

    const validateProcess = cp.spawn('ivy-validate', ['--json', filePath], {
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
