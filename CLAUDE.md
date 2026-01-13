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
└── render/              # 描画
    ├── image.rs         # 背景・スプライト描画
    ├── text.rs          # テキストボックス
    └── ui.rs            # 選択肢ボタン

assets/                  # ゲームアセット
├── *.yaml               # シナリオファイル
└── *.png                # 背景・スプライト画像

saves/                   # セーブデータ
└── save.json            # クイックセーブ
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
```

## 操作

- クリック / Enter: テキスト進行
- 選択肢クリック: 分岐
- F5: クイックセーブ
- F9: クイックロード
- Escape: 終了

## 依存クレート

| クレート | 用途 |
|---------|------|
| macroquad | 2Dレンダリング、画像読み込み、WASM対応 |
| serde + serde_yaml | YAMLパース |
| serde_json | セーブデータ |
| anyhow | エラー処理 |

## 実装状況

### Phase 1（完了）
- [x] YAMLシナリオパーサー
- [x] テキスト表示
- [x] 選択肢と分岐（jump）
- [x] ラベル定義

### Phase 2（完了）
- [x] 背景画像表示
- [x] キャラクタースプライト表示
- [x] スプライト位置指定（left/center/right）
- [x] 画像キャッシュ
- [x] 画像状態の継続（前の画像を維持）

### Phase 3（完了）
- [x] セーブ/ロード（F5: セーブ、F9: ロード）

### 今後の予定
- [ ] ロールバック
- [ ] 演出（トランジション、フェード）
- [ ] 日本語フォント対応
- [ ] WASM対応
