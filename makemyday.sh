#!/usr/bin/env sh
set -e

[ -z "$1" ] && echo "Usage: ./makemyday.sh [day]"

DIR="day$1"
sed -i '$ d' Cargo.toml
printf '    \"%s\",\n]' "$DIR" >> Cargo.toml
cargo new "$DIR"
sed "s/%DIR%/$DIR/g" .base/main.rs > "$DIR/src/main.rs"
mkdir "$DIR/input"
touch "$DIR/input/input.txt"