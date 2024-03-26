#!/usr/bin/env bash

TARGET='https://raw.githubusercontent.com/google/mozc/master/src/data/emoticon/emoticon.tsv'
INPUT='emoticon.tsv'
OUTPUT='skk-emoticon.utf8'

curl $TARGET -o $INPUT

if [ $? != 0 ]; then
    echo '辞書ファイルのダウンロード中に問題が発生しました' 1>&2
    exit
fi

cat $INPUT | rust-script mozc-emoticon-converter.rs > $OUTPUT
