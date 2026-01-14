# Development Tools

IDE support and tooling for ivy scenario development.

## Overview

Development tools to improve the authoring experience for ivy scenarios, including syntax highlighting, validation, and preview functionality.

## Components

### VSCode Extension

A Visual Studio Code extension for ivy YAML scenarios.

#### Features

- **Syntax highlighting**: Color coding for ivy-specific YAML syntax
- **Auto-completion**: Suggest commands, labels, and asset paths
- **Snippets**: Quick templates for common patterns
- **Diagnostics**: Real-time error detection

#### Implementation

```json
{
  "name": "ivy-vscode",
  "contributes": {
    "languages": [{
      "id": "ivy-scenario",
      "extensions": [".yaml", ".yml"],
      "configuration": "./language-configuration.json"
    }],
    "grammars": [{
      "language": "ivy-scenario",
      "scopeName": "source.ivy",
      "path": "./syntaxes/ivy.tmLanguage.json"
    }]
  }
}
```

### Scenario Validator

CLI tool for checking scenario correctness.

#### Checks

- Undefined label references in `jump` commands
- Missing asset files (images, audio)
- Syntax errors in YAML
- Unused labels
- Circular jump detection

#### Usage

```bash
ivy validate scenario.yaml
ivy validate --all assets/
```

### Real-time Preview

Preview scenarios directly in VSCode.

#### Features

- Webview panel showing current scene
- Hot reload on file save
- Navigate through script with keyboard
- Variable state inspection

## References

- [Novelscript VSCode Extension](https://marketplace.visualstudio.com/items?itemName=nobbele.novelscript-vscode)
- [Visual Novel Maker Script Editor](https://asset.visualnovelmaker.com/help/GameScript_Getting_Started.htm)
