#!/bin/bash
set -ex

cargo install svd2rust form
rm -rf src/*
svd2rust -c svd2rust.toml -i bcm2837_lpa.svd -o src
form -i src/lib.rs -o src
cargo fmt