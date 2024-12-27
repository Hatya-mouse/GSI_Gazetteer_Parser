# GSI Gazetteer Parser

**日本語の情報は下にあります。**

A Rust-based command-line tool that extracts geographical names from GSI (国土地理院, Geospatial Information Authority of Japan) Gazetteer PDFs. This tool parses municipality and populated area names in both Japanese and English, making the data easily accessible in CSV format.

## Features

- Extracts municipality and populated area names from GSI gazetteer PDF
- Processes both Japanese and English geographical names
- Exports data in CSV format for easy integration with other tools

## Requirements

Dependencies:
* lopdf: PDF parsing
* anyhow: Error handling

## Installation

1. Clone
```bash
git clone https://github.com/Hatya-mouse/GSI_Gazetteer_Parser.git
cd gsi_gazetteer_parser
```

2. Build
```bash
cargo build --release
```

## Usage

Run the tool by specifying the PDF file path and **the page number of the start of the list**. Additional options are available for customizing the output:

```bash
cargo run -- <path-to-the-file> <page-number> [options]

Options:
-r, --remove-suffix    Remove municipality suffixes (e.g., Shi, Machi)
-o, --output-path <path>    Specify custom output path for the CSV file
```

Example:
```bash
cargo run -- ./gazetteer.pdf 8 --remove-suffix --output-path ./output/names.csv
```

## Output Format

The tool generates a CSV file with the following columns:
- Japanese Name (ja): The geographical name in Japanese
- English Name (en): The corresponding English name

Example output：
```csv
ja,en
東京,Tokyo
横浜,Yokohama
札幌,Sapporo
```

---

# 国土地理院「地名集日本」PDFデータ解析ツール

国土地理院が発行する[「地名集日本」](https://www.gsi.go.jp/kihonjohochousa/gazetteer.html)のPDFファイルから地名データを抽出するRust製のコマンドラインツールです。市区町村名や居住地域名を日本語・英語の両方で解析し、CSVフォーマットで簡単にアクセスできるようにデータを変換します。日本語地名の翻訳などにお役立てください。

## 機能

- 国土地理院の地名集のPDFファイルから、`Municipality`（行政）または`Populated Area`（居住地）の要素のみを抽出し、CSVファイルを生成します。
- 日本語と英語、両方の地名を抽出します。
- 解析に最適なCSVフォーマットを生成します。

## 要件

依存関係：
- lopdf: PDFパース
- anyhow: エラー処理

## インストール

1. リポジトリのクローン
```bash
git clone https://github.com/Hatya-mouse/GSI_Gazetteer_Parser.git
cd gsi_gazetteer_parser
```

2. ビルド
```bash
cargo build --release
```

## 使用

PDFのファイルへのパスと、地名リストの**最初の**ページ番号を設定して実行します。以下のオプションで出力をカスタマイズできます：

```bash
cargo run -- <path-to-the-file> <page-number> [options]

オプション：
-r, --remove-suffix    「市」「村」「県」などの接尾辞を除去します。
-o, --output-path <path>    CSVファイルの出力先を指定します。このオプションを設定しなかった場合は、カレントディレクトリに出力されます。
```

例：
```bash
cargo run -- ./gazetteer.pdf 8 --remove-suffix --output-path ./output/names.csv
```

このツールは以下の情報を含むCSVファイルを生成します。
* 日本語名 (ja): 日本語での地名（-r/--remove-suffixオプションで市区町村などの接尾辞を除去可能）
* 英語名 (en): 対応する英語名

## 出力フォーマット

- 日本語の地名（ja）：「市」「町」「村」などを除いた、日本語の地名
- ローマ字の地名（en）："Shi" "Machi"などを除いた、ローマ字表記の地名

出力例：
```csv
ja,en
東京,Tokyo
横浜,Yokohama
札幌,Sapporo
```
