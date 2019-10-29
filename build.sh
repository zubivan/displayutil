#!/bin/bash

set -ex

VERSION=${1:-"XXX"}

create_release_package() {
    local target="x86_64-apple-darwin"
    local name="displayutil-${VERSION}-${target}"

    cargo build --target "$target" --release

    local out_dir="out/$name"
    mkdir -p "$out_dir"

    cp "target/$target/release/displayutil" "$out_dir/displayutil"

    tar czf "out/$name.tar.gz" "$out_dir"
}

create_release_package