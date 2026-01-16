# ivy

Rust製ビジュアルノベルエンジン。

## 設計思想

- **Simple > Easy**: Rich Hickeyの「Simple Made Easy」に基づき、長期的な保守性を重視
- **レイヤー分離**: データ層・変換層・ロジック層・表示層が独立し、互いの内部実装を知らない
- **最小限の依存**: 必要なクレートのみ使用

## アーキテクチャ

```
シナリオ（YAML） → パーサー → ランタイム → レンダラー
```

## ディレクトリ構成

```
src/
├── main.rs              # エントリポイント、画像キャッシュ管理
├── scenario/            # シナリオ関連
│   ├── types.rs         # Scenario, Command, Choice, CharPosition
│   └── parser.rs        # YAML読み込み
├── runtime/             # 実行エンジン
│   └── state.rs         # GameState, DisplayState, VisualState, SaveData
├── render/              # 描画
│   ├── image.rs         # 背景・スプライト描画
│   ├── text.rs          # テキストボックス
│   ├── transition.rs    # トランジション効果
│   └── ui.rs            # 選択肢ボタン
├── hotreload/           # ホットリロード（開発用）
│   └── mod.rs           # ファイル監視、シナリオ再読込
├── i18n/                # 多言語対応
│   ├── mod.rs           # 翻訳システム
│   └── localized.rs     # LocalizedString型
├── modding/             # コミュニティMod対応
│   ├── mod.rs           # モジュールエントリ
│   └── types.rs         # ModInfo, ModLoader
└── input/               # 入力処理
    ├── mod.rs           # InputProviderトレイト
    └── gamepad.rs       # ゲームパッド定義

assets/                  # ゲームアセット
├── *.yaml               # シナリオファイル
└── *.png                # 背景・スプライト画像

saves/                   # セーブデータ
└── save.json            # クイックセーブ

tests/                   # テスト
├── fixtures/            # テスト用YAMLシナリオ
├── snapshots/           # Instaスナップショット
├── e2e/                 # Playwright E2Eテスト
│   ├── specs/           # テストファイル
│   └── helpers/         # ヘルパー関数
├── snapshot_test.rs     # DisplayStateスナップショット
├── state_test.rs        # GameState統合テスト
├── parser_test.rs       # YAMLパーサーテスト
├── variables_test.rs    # 変数システムテスト
├── keybinds_test.rs     # キーバインドテスト
└── integration_test.rs  # シナリオ実行テスト

editors/                 # エディタ拡張
└── vscode/              # VSCode拡張
    ├── package.json     # 拡張メタデータ
    ├── syntaxes/        # シンタックスハイライト
    └── snippets/        # コードスニペット
```

## シナリオ形式（YAML）

```yaml
title: Scene Title

script:
  - text: "テキストのみ"

  - background: "assets/bg.png"
    text: "背景を設定"

  - character: "assets/char.png"
    char_pos: "center"        # left / center / right
    text: "キャラクターを表示"

  - text: "画像指定なし → 前の画像を維持"

  - background: ""            # 空文字 → 背景をクリア
    character: ""             # 空文字 → キャラクターをクリア
    text: "画像をクリア"

  - text: "選択肢"
    choices:
      - label: "選択肢1"
        jump: label_name
      - label: "選択肢2"
        jump: other_label

  - label: label_name
    text: "ラベル付きテキスト"

  - jump: ending              # 無条件ジャンプ

  # NVLモード（全画面テキスト表示）
  - nvl: true                   # NVLモードに切り替え
    text: "NVLモードのテキスト"

  - text: "テキストが画面に累積表示される"

  - nvl_clear: true             # NVLバッファをクリア（新しいページ）
    text: "新しいページのテキスト"

  - nvl: false                  # ADVモードに戻る
    text: "通常のテキストボックス表示"

  # モジュラーキャラクター（レイヤード スプライト合成）
modular_characters:
  sakura:
    base: "assets/characters/sakura/base.png"
    layers:
      - name: "hair"
        images:
          - "assets/characters/sakura/hair_normal.png"
          - "assets/characters/sakura/hair_wind.png"
      - name: "expression"
        images:
          - "assets/characters/sakura/expr_neutral.png"
          - "assets/characters/sakura/expr_smile.png"
      - name: "outfit"
        images:
          - "assets/characters/sakura/outfit_school.png"
          - "assets/characters/sakura/outfit_casual.png"

script:
  - modular_char:
      name: sakura
      expression: 0             # レイヤーのバリアント番号
      outfit: 0
    char_pos: center
    text: "モジュラーキャラクター"

  - modular_char:
      name: sakura
      expression: 1             # 表情を変更
    text: "表情だけ変更"

  # ダイナミックカメラ（パン、ズーム、チルト）
  - camera:
      zoom: 1.5               # ズームレベル（1.0 = 通常）
      duration: 1.0           # アニメーション時間
      easing: ease_out_quad   # イージング関数
    text: "ズームイン"

  - camera:
      pan:
        x: 100                # X方向オフセット（ピクセル）
        y: 50                 # Y方向オフセット（ピクセル）
      duration: 0.5
    text: "パン移動"

  - camera:
      tilt: 5                 # 傾き（度）
      duration: 0.3
    text: "カメラ傾斜"

  - camera:
      pan: { x: 0, y: 0 }     # 複合エフェクト
      zoom: 1.0
      tilt: 0
      focus: center           # フォーカス点
      duration: 1.5
    text: "通常に戻る"

  # レイヤードオーディオ（アンビエント音の重ね合わせ）
  - ambient:
      - id: rain              # トラック識別子
        path: "assets/audio/rain.ogg"
        volume: 0.6           # 音量（0.0-1.0）
        looped: true          # ループ再生
        fade_in: 0.5          # フェードイン秒数
    text: "雨が降り始める..."

  - ambient:
      - id: thunder
        path: "assets/audio/thunder.ogg"
        volume: 0.4
        looped: false         # 一度だけ再生
    text: "雷が鳴る"

  - ambient_stop:
      - id: rain
        fade_out: 1.0         # フェードアウト秒数
    text: "雨が止む"

  # 動画再生（--features video が必要）
  - video:
      path: "assets/videos/opening.webm"
      skippable: false        # スキップ不可
      bgm_fade_out: 1.0       # BGMフェードアウト秒数

  # 動画背景（--features video が必要）
  - video_bg:
      path: "assets/videos/forest_loop.webm"
      looped: true            # ループ再生（デフォルト: true）
    text: "動画が背景として再生"

  - video_bg:
      path: ""                # 空文字 → 動画背景を停止
    text: "静止画背景に戻る"
```

### 画像状態の継続ルール

- フィールドなし → 前の状態を維持
- 空文字 `""` → クリア（非表示）
- パス指定 → 新しい画像を表示

## コマンド

```bash
cargo run              # 実行
cargo build            # ビルド
cargo build --release  # リリースビルド

# 動画機能付きビルド（FFmpeg必要）
cargo run --features video
cargo build --features video

# テスト
cargo test             # 全テスト実行
cargo test --lib       # 単体テストのみ
cargo test --test '*'  # 統合テストのみ
cargo insta test       # スナップショットテスト
cargo insta review     # スナップショット確認・承認

# E2Eテスト
cd tests/e2e && npm test           # E2Eテスト実行
cd tests/e2e && npm run test:update  # スナップショット更新

# シナリオバリデーション
cargo run --bin ivy-validate -- scenario.yaml          # 単一ファイル検証
cargo run --bin ivy-validate -- --all assets/          # ディレクトリ内全ファイル検証
cargo run --bin ivy-validate -- --watch assets/        # ファイル監視モード（変更時自動検証）
cargo run --bin ivy-validate -- --cycles scenario.yaml # 循環パス検出も実行
cargo run --bin ivy-validate -- --no-color scenario.yaml # カラー出力無効
cargo run --bin ivy-validate -- --json scenario.yaml   # JSON形式で出力（CI/ツール連携用）
cargo run --bin ivy-validate -- --quiet scenario.yaml  # エラーのみ出力（警告・情報を抑制）

# リアルタイムプレビュー
cargo run --bin ivy-preview -- scenario.yaml           # プレビューサーバー起動（http://127.0.0.1:3000）
cargo run --bin ivy-preview -- --port 8080 scenario.yaml # カスタムポート指定

# Language Server (LSP)
cargo build --bin ivy-lsp                              # LSPサーバーをビルド
# VSCode拡張が自動検出、または ivy.lspPath で設定
```

## 操作

### キーボード / マウス
- クリック / Enter: テキスト進行
- 選択肢クリック: 分岐
- A: オートモード切替
- S: スキップモード切替
- L: バックログ表示
- ↑ / マウスホイール上: ロールバック
- F5: クイックセーブ
- F9: クイックロード
- F12: デバッグコンソール
- Shift+1-9: スロットセーブ
- 1-9: スロットロード
- Escape: タイトルへ戻る / 終了

### ゲームパッド
- A: テキスト進行
- B: キャンセル
- X: オートモード切替
- Y: スキップモード切替
- LB: ロールバック
- Select: バックログ表示
- Start: メニュー

## 依存クレート

| クレート | 用途 |
|---------|------|
| macroquad | 2Dレンダリング、画像読み込み、WASM対応 |
| serde + serde_yaml | YAMLパース |
| serde_json | セーブデータ |
| anyhow | エラー処理 |
| notify | ファイル監視（ネイティブのみ、ホットリロード用） |
| gamepads | ゲームパッド入力（ネイティブのみ） |
| video-rs | 動画再生（ネイティブのみ、optional、FFmpeg必要） |

## 実装状況

### 基本機能（完了）
- [x] YAMLシナリオパーサー
- [x] テキスト表示（タイプライター効果）
- [x] 選択肢と分岐（jump）
- [x] ラベル定義
- [x] 背景画像表示
- [x] キャラクタースプライト表示（left/center/right）
- [x] 複数キャラクター表示
- [x] モジュラーキャラクター（レイヤード スプライト合成）
- [x] 画像キャッシュ
- [x] 画像状態の継続

### セーブ/ロード
- [x] クイックセーブ/ロード（F5/F9）
- [x] 複数スロット（1-10）
- [x] ロールバック（履歴50件）
- [x] バックログ表示

### オーディオ
- [x] BGM再生（ループ、フェード）
- [x] 効果音（SE）
- [x] ボイス再生
- [x] レイヤードオーディオ（環境音の重ね合わせ）

### 演出
- [x] フェードトランジション
- [x] 画面シェイク
- [x] キャラクター入退場アニメーション
- [x] パーティクルエフェクト（雪、雨、桜、etc.）
- [x] シネマティックバー（レターボックス）
- [x] イージング関数（14種類）
- [x] トランジション拡張（Wipe, Slide, Pixelate, Iris, Blinds）
- [x] ダイナミックカメラ（パン、ズーム、チルト）

### テキスト機能
- [x] 色付きテキスト `{color:red}text{/color}`
- [x] ルビ（振り仮名） `{ruby:漢字:かんじ}`
- [x] 変数展開 `{var:name}`
- [x] プレイヤー名入力
- [x] NVLモード（全画面テキスト表示）

### ゲームシステム
- [x] タイトル画面
- [x] 設定画面（音量、テキスト速度、オート速度）
- [x] オートモード / スキップモード
- [x] 時間制限付き選択肢
- [x] 変数システム（set/if）
- [x] CGギャラリー
- [x] 実績システム
- [x] チャプターセレクト
- [x] デバッグコンソール
- [x] カスタマイズ可能なキーバインド

### プラットフォーム
- [x] ネイティブ（macOS, Windows, Linux）
- [x] WASM対応（platform抽象化）
- [x] 日本語フォント対応

### 開発者向け
- [x] ホットリロード（YAMLシナリオ自動再読込）
- [x] コントローラー対応（`gamepads`クレート統合済み）
- [x] 多言語対応（LocalizedString型、Command型統合済み）
- [x] フローチャート表示（シナリオ構造の可視化）

### 演出（追加）
- [x] 動画再生（ネイティブ: FFmpeg、WASM: HTML5 video）※ `--features video` で有効化
- [x] 動画背景（ループ動画を背景として表示）※ `--features video` で有効化

### アクセシビリティ
- [x] フォントサイズ調整（50% - 200%）
- [x] ハイコントラストモード
- [x] 行間調整（1.0x - 2.0x）
- [x] 字間調整（-2.0px - 5.0px）
- [x] ディスレクシアフォント対応（OpenDyslexic）
- [x] セルフボイシング（スクリーンリーダー対応）
  - TTS モード（WASM: Web Speech API）
  - クリップボードモード（外部スクリーンリーダー連携）

### 開発ツール
- [x] シナリオバリデーター（`ivy-validate` CLI）
  - 未定義ラベルへの参照検出
  - 重複ラベル検出
  - 未使用ラベル警告
  - 自己参照ジャンプ検出
  - 循環パス検出
  - ファイル監視モード（`--watch`）
  - カラー出力（`--no-color` で無効化可能）
  - JSON出力（`--json`、CI/ツール連携用）
  - 静音モード（`--quiet`、エラーのみ出力）
- [x] リアルタイムプレビュー（`ivy-preview` CLI）
  - WebSocket ベースのライブプレビュー
  - ファイル変更時の自動リロード
  - コマンドナビゲーション（前後移動、ラベルジャンプ）
  - 変数状態の表示
  - NVL/ADVモード表示
- [x] VSCode拡張（`editors/vscode`）
  - シンタックスハイライト（`.ivy.yaml`, `.ivy.yml`）
  - コードスニペット（40+ パターン）
  - プレビューコマンド（`Ivy: Open Preview`）
  - バリデーションコマンド（`Ivy: Validate Scenario`）
  - CLIバイナリ自動検出・インストールガイド

### コミュニティModding
- [x] Modローダー（`src/modding`）
  - Mod発見・読み込み（`mods/` ディレクトリ）
  - Modメタデータ（`mod.yaml`）
  - Mod種別（scenario, characters, translation, assets, patch）
  - 優先度によるロード順管理
  - 有効/無効の切り替え

### 演出（追加）
- [x] リップシンク（`src/render/lipsync.rs`）
  - タイミングベースの口パクアニメーション
  - 複数キャラクター対応（LipSyncManager）
  - 設定可能な速度・開度

### 保留中の機能

以下の機能は技術的・ライセンス的な制約により保留中。

- Live2D対応: Cubism SDK（C++）とライセンスが必要
- Spine対応: Spine Runtime とライセンスが必要
- クラウドセーブ: バックエンドサーバーの構築・運用が必要

### 非エンジニア向けロードマップ

ivy を「非エンジニアでもVNゲームを作れる」エンジンにするための改善計画。

#### Phase 1: ドキュメント整備
- [x] Getting Started ガイド（Rust インストール、ビルド、Hello World）
- [x] よくあるエラーの FAQ
- [x] YAML 構文ガイド（初心者向け）

#### Phase 2: 配布の簡素化
- [x] GitHub Releases にビルド済みバイナリを配布（Windows / macOS / Linux）
- [x] GitHub Actions による自動ビルド
- [x] WASM デモサイトの公開

#### Phase 3: エラー体験の改善
- [x] YAML パースエラーに行番号を追加
- [x] よくあるエラーパターンの検出とヒント表示

#### Phase 4: ツール統合の強化
- [x] VSCode 拡張: CLI バイナリの自動検出・インストールガイド
- [x] Language Server Protocol (LSP) 対応
  - Diagnostics（リアルタイムバリデーション）
  - Go to Definition（ラベルジャンプ）
  - Find References（参照検索）
  - Completion（キーワード、ラベル、char_pos、easing）
  - Hover（フィールドドキュメント）

#### Phase 5: ビジュアルエディタ

非エンジニアでも YAML を知らずにビジュアルノベルを作成できる GUI エディタ。
技術スタック: Tauri + React + TypeScript

- [x] Phase 5.1: コマンドエディタ MVP
  - [x] Tauri プロジェクト scaffold (`editors/ivy-editor/`)
  - [x] YAML 読み込み・保存（既存パーサー再利用）
  - [x] コマンドリスト表示（テーブル形式）
  - [x] 単一コマンド編集フォーム
  - [x] リアルタイム YAML プレビュー（読み取り専用）
  - [x] バリデーションエラー表示
- [x] Phase 5.2: フローチャート表示
  - [x] 既存 `Flowchart` 型を React Flow で描画
  - [x] ノードクリックでコマンドにジャンプ
  - [x] ズーム・パン操作
- [x] Phase 5.3: プレビュー統合（完全統合方式）
  - [x] `PreviewState` を Tauri コマンドとして実装
  - [x] プレビューレンダリングを React コンポーネントに移植
  - [x] エディタ ↔ プレビューの同期
- [x] Phase 5.4: ドラッグ＆ドロップ
  - [x] コマンドリストの順序変更
  - [x] アセットファイルのドロップでパス自動入力
- [x] Phase 5.5: アセット管理
  - [x] アセットブラウザ（ファイルツリー）
  - [x] 画像・音声プレビュー
  - [x] 未使用アセット検出

#### Phase 6: プロジェクト管理

- [x] プロジェクト作成ウィザード
- [x] プロジェクト設定画面（タイトル、解像度、作者情報）
- [x] マルチシナリオ/チャプター管理
- [x] 最近開いたプロジェクト一覧

#### Phase 7-11: 完全開発ツール化（長期目標）

ivy-editor を VN ゲーム開発の全工程をカバーするツールに拡張する。

- [x] Phase 7: キャラクターシステム
  - [x] キャラクターデータベース UI
  - [x] モジュラーキャラクターの GUI 設定
  - [x] スピーカー名との自動関連付け
- [x] Phase 8: 高度なコマンドエディタ
  - [x] トランジション効果ビジュアルピッカー
  - [x] カメラコマンド GUI（パン/ズーム/チルト）
  - [x] パーティクルエフェクトプレビュー
  - [x] 変数エディタ（set/if の視覚的編集）
- [x] Phase 9: 多言語サポート
  - [x] LocalizedString の GUI 編集
  - [x] 翻訳テーブル管理画面
  - [x] 言語切替プレビュー
- [ ] Phase 10: テスト＆デバッグ
  - [x] エディタ内プレイテストモード
  - [x] 変数ウォッチャー/デバッガー
  - [x] ストーリーパス分析（未到達分岐検出）
  - [ ] セーブデータ検証ツール
- [ ] Phase 11: ビルド＆配布
  - [ ] エクスポートウィザード（Windows/macOS/Linux/Web）
  - [ ] アセット最適化（画像圧縮、音声変換）
  - [ ] リリースパッケージ生成
