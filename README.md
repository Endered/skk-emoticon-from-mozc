# これはなに
SKKで顔文字(例えば`(´・ω・`)`)を使用するための辞書を生成するスクリプトです。

入力データとしてmozcで使用しているファイルを使用しています。

これ→ https://github.com/google/mozc/blob/master/src/data/emoticon/emoticon.tsv

# 使い方
事前に以下の用意をした上で`bash main.sh`をプロジェクト直下で実行してください
- curl
- [rust-script](https://rust-script.org/)

具体的な動作は`mozc-emoticon-converter.rs`を参照してください。
