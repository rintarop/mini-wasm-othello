# Mini WASM Othello

Rust + WebAssemblyで作成したシンプルなオセロゲームです。

## 特徴

- Rustで実装されたゲームロジック
- WebAssemblyを使用してブラウザで動作
- HTML5 Canvasを使用したシンプルなUI
- GitHub Pagesでの公開に対応

## 開発環境のセットアップ

### 必要なツール

1. Rust (https://rustup.rs/)
2. wasm-pack (https://rustwasm.github.io/wasm-pack/installer/)
3. Node.js (npmが使用できること)

### セットアップ手順

1. このリポジトリをクローンまたはダウンロード
2. 依存関係をインストール:
   ```bash
   npm install
   ```

3. プロジェクトをビルド:
   ```bash
   npm run build
   ```

4. 開発サーバーを起動:
   ```bash
   npm run serve
   ```

5. ブラウザで `http://localhost:8000` にアクセス

## ビルドとデプロイ

### ローカルでのビルド

```bash
npm run build
```

これにより `pkg/` ディレクトリにWebAssemblyファイルが生成されます。

### GitHub Pagesでの公開

1. GitHubリポジトリを作成
2. コードをプッシュ
3. GitHub Actionsワークフローが自動でビルドとデプロイを実行
4. GitHub PagesでWebサイトが公開されます

## ゲームの遊び方

1. 黒（最初のプレイヤー）から開始
2. 空いているマスをクリックして石を置く
3. 相手の石を挟むように石を置くとひっくり返る
4. 交互にプレイして最終的に石の数が多い方が勝利

## 技術スタック

- **Rust**: ゲームロジックの実装
- **WebAssembly (wasm-bindgen)**: Rustコードをブラウザで実行
- **HTML5 Canvas**: ゲーム盤の描画
- **JavaScript (ES6 modules)**: WebAssemblyとDOMの連携
- **GitHub Actions**: 自動ビルドとデプロイ

## ライセンス

MIT License
