# WIP!!

_現在誠意作成中です！_
続報があり次第、[ツイッター](https://x.com/sota_eng_prog) or [Qiita](https://qiita.com/sotanengel) で宣言していくので少々お待ちくださいませ。

# Effective-Rustlings

Rustlings の**日本語版サードパーティー**へようこそ 😃
不備などございましたら、[こちら](https://github.com/sotanengel/rustlings-jp/issues)から連絡くださいませ。

# 導入方法

※ 仕様の更新などで手順に変更が加わる可能性があるため、問題が生じた場合には[本家](https://github.com/rust-lang/rustlings/blob/main)などを確認し、
[こちら](https://github.com/sotanengel/rustlings-jp/issues)で修正内容を報告いただけますと幸いです。

## 1. Rust をインストールする

まず[www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)にアクセスし、最新バージョンの Rust をインストールしましょう。

> 🐧 もしも Linux であれば, `gcc`をインストールしましょう。
>
> Deb: `sudo apt install gcc`.
> Dnf: `sudo dnf install gcc`.

> 🍎 もしも MacOS を使っている場合は,Xcode とデベロッパーツールを`xcode-select --install`でインストールしてください。

## 2. Rustlings をインストールする

Rustlings の演習をスムーズに進めるためのコマンドラインツールをターミナル上からインストールします。

```bash
cargo install rustlings
```

<details>
<summary><strong>もしも失敗した場合には…</strong> (<em>詳細を開く</em>)</summary>

- `rustup update`コマンドで最新バージョンの Rust か確認してください
- `--locked`フラグを利用してみてください、こんな感じで →`cargo install rustlings --locked`
- もしくは[本家の issue](https://github.com/rust-lang/rustlings/issues/new)で報告してください

</details>

## 3.演習問題のダウンロード

最新バージョンの日本語の演習問題をダウンロードしましょう！
ダウンロードした zip ファイルを解凍し、好きな場所に配置してください。

[ダウンロード](https://github.com/sotanengel/rustlings-jp/tree/20240829)

## 4. 演習問題にチャレンジ！

ターミナル上で解凍した`rustlings-jp-YYYYMMDD`のディレクトリに移動し、以下のコマンドを実行してください。

```bash
rustlings
```

<details>
<summary><strong>もしも「<code>rustlings</code>コマンドが見つからない」とターミナル上で警告が出た場合には…</strong> (<em>詳細を開く</em>)</summary>

もしも Linux 系を使っており、Rust をパッケージマネージャーでインストールしていた場合には、
Cargo が`~/.cargo/bin`にダウンロードされているものの、
`~/.cargo/bin`が`PATH`の環境変数に入っていない可能性があります。

解決方法としては、

- 手動で`~/.cargo/bin`を`PATH`に追加する
- Rust をアンインストールし、`rustup`: https://www.rust-lang.org/tools/install でインストールする

</details>

# 操作方法の簡単な説明

- `rustlings`：問題集を解くためのツールが起動する
- `n`：次の問題に進む
- `l`：問題のリスト一覧を表示する
  - `c`：カーソルで合わせた問題から演習を再開する
- `r`：問題の回答ステータスをリセットする
