#!/bin/bash

# Bump the release tag
read -p "Release version as x.x.x: " tag
dasel put -r toml -t string -v $tag 'version'

# Build release binaries
cargo fmt
cargo build --release
tar -czvf ./dist/nextup-Linux-x86_64.tar.gz target/release/nextup

# Generate the GitHub release
read -p "Publish the released version? (Y/N): " confirm && [[ $confirm == [yY] || $confirm == [yY][eE][sS] ]] || exit 1
gh release create "v${tag}" ./dist/*.tar.gz -F changelog.md

