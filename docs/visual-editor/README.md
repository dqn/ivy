# Phase 5: Visual Editor

Non-engineers can create visual novels without YAML knowledge.

## Technology Stack

| Component | Technology | Reason |
|-----------|------------|--------|
| Desktop Framework | Tauri 2.x | Small binary (~10MB), native Rust integration |
| Frontend | React 18 + TypeScript | Rich ecosystem, VSCode extension compatibility |
| Build Tool | Vite | Fast HMR, TypeScript support |
| Type Generation | ts-rs | Rust → TypeScript automatic type generation |

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    Tauri App                             │
├─────────────────────────────────────────────────────────┤
│  React Frontend                                          │
│  ┌────────────┐ ┌────────────┐ ┌────────────────────┐   │
│  │ Command    │ │ Flowchart  │ │ Preview            │   │
│  │ Editor     │ │ View       │ │ Panel              │   │
│  └────────────┘ └────────────┘ └────────────────────┘   │
│                        │ IPC                             │
│  Rust Backend                                            │
│  ┌────────────┐ ┌────────────┐ ┌────────────────────┐   │
│  │ ivy-core   │ │ File I/O   │ │ Preview State      │   │
│  │ (parser,   │ │            │ │                    │   │
│  │ validator) │ │            │ │                    │   │
│  └────────────┘ └────────────┘ └────────────────────┘   │
└─────────────────────────────────────────────────────────┘
```

## Directory Structure

```
editors/
├── vscode/              # Existing VSCode extension
└── ivy-editor/          # New Tauri app
    ├── Cargo.toml
    ├── tauri.conf.json
    ├── src/
    │   ├── main.rs
    │   └── commands/
    │       ├── mod.rs
    │       ├── scenario.rs
    │       ├── preview.rs
    │       └── assets.rs
    └── ui/
        ├── package.json
        ├── tsconfig.json
        ├── vite.config.ts
        └── src/
            ├── App.tsx
            ├── components/
            │   ├── CommandList/
            │   ├── CommandForm/
            │   ├── FlowchartView/
            │   ├── PreviewPanel/
            │   └── AssetBrowser/
            ├── hooks/
            │   ├── useScenario.ts
            │   └── usePreview.ts
            └── types/
                └── scenario.ts
```

## Phases

| Phase | Description | Document |
|-------|-------------|----------|
| 5.1 | Command Editor MVP | [phase-5.1-command-editor.md](./phase-5.1-command-editor.md) |
| 5.2 | Flowchart View | [phase-5.2-flowchart.md](./phase-5.2-flowchart.md) |
| 5.3 | Preview Integration | [phase-5.3-preview.md](./phase-5.3-preview.md) |
| 5.4 | Drag & Drop | [phase-5.4-drag-and-drop.md](./phase-5.4-drag-and-drop.md) |
| 5.5 | Asset Management | [phase-5.5-asset-management.md](./phase-5.5-asset-management.md) |

## Design Principles

### Simple > Easy

1. **Minimal UI**
   - Initial view: Command list + basic form only
   - Advanced features in collapsible sections

2. **Progressive Complexity Disclosure**
   - Beginner: text, speaker, background, character
   - Intermediate: choices, jump, set, if
   - Advanced: transition, camera, particles

3. **YAML Transparency**
   - Always show "View YAML" button
   - Text editor mode available for power users

4. **Fallback**
   - Unsupported fields displayed as raw YAML

## Key Files to Reference

| File | Purpose |
|------|---------|
| `src/scenario/types.rs` | Command, Scenario types (TypeScript generation source) |
| `src/scenario/parser.rs` | YAML parser (reuse) |
| `src/scenario/validator.rs` | Validation logic (reuse) |
| `src/flowchart/builder.rs` | Flowchart generation (reuse) |
| `src/bin/preview.rs` | Preview server (reference) |
| `editors/vscode/snippets/ivy.json` | All command snippets (form field reference) |

## Dependencies

### Rust (Cargo.toml additions)

```toml
[dependencies]
tauri = { version = "2", features = ["shell-open"] }
ts-rs = "10"

[build-dependencies]
tauri-build = "2"
```

### Frontend (package.json)

```json
{
  "dependencies": {
    "@tauri-apps/api": "^2",
    "react": "^18",
    "react-dom": "^18",
    "@xyflow/react": "^12",
    "dagre": "^0.8",
    "@dnd-kit/core": "^6",
    "@dnd-kit/sortable": "^8"
  },
  "devDependencies": {
    "@tauri-apps/cli": "^2",
    "typescript": "^5",
    "vite": "^5",
    "@vitejs/plugin-react": "^4"
  }
}
```

## Success Criteria

### Phase 5.1 MVP Completion

- [ ] Create new scenario using GUI only
- [ ] Open and edit existing scenario
- [ ] Validation errors displayed in GUI
- [ ] Non-engineer can learn basic operations within 30 minutes
