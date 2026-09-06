#!/usr/bin/env bash
set -euo pipefail

if [ $# -lt 1 ]; then
  echo "Usage: $0 topic_name"
  exit 1
fi

name="$*"
name="${name//[[:space:]]/_}"
dir="src/bin"
file="$dir/${name}.rs"

mkdir -p "$dir"

if [ -e "$file" ]; then
  echo "File $file already exists"
  exit 1
fi

cat > "$file" <<EOF
// Topic: $name 
// Run with:  cargo run --bin $name

fn main() {
    println!("Hello World : cargo run --bin $name");
}
EOF

echo "Created $file"
