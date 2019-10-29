#!/bin/bash

set -ex

create_release_package() {
    local target="x86_64-apple-darwin"
    local name="displayutil-xxx-${target}"

    cargo build --target "$target" --release

    local out_dir="out/$name"
    mkdir -p "$out_dir"

    cp "target/$target/release/displayutil" "$out_dir/displayutil"

    tar czf "out/$name.tar.gz" "$out_dir"
}

create_release_package