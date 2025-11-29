#!/usr/bin/env bash

set -euo pipefail

template_path="src/template.rs"
day_num=$(printf "%02d" "$1")
day_dir="src/d${day_num}"

set -x
if [[ ! -d "$day_dir" ]]; then
    mkdir -p "$day_dir"
    cp "$template_path" "$day_dir/mod.rs"
fi
touch "inputs/day${day_num}-1.txt"
touch "inputs/day${day_num}-2.txt"