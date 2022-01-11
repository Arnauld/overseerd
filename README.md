# overseer-d

[![GitHub license](https://img.shields.io/github/license/Arnauld/overseerd.svg)](https://github.com/Arnauld/overseerd/blob/master/LICENSE)
[![GitHub release](https://img.shields.io/github/release/Arnauld/overseerd.svg)](https://GitHub.com/Arnauld/overseerd/releases/)

## Dev. env

      docker run -v $(pwd):/home/rust/src -it ekidd/rust-musl-builder:1.51.0 /bin/bash

      cargo build
      cargo test

## Building and Running a Cargo Project

      cargo build

## Building for Release

      cargo build --release

## Docker (or without rust env.) build

      docker build -t technbolts/jwtd .

      docker tag -i 7358d9f4b652 technbolts/jwtd:0.1.0
      docker login -u xxxx -p xxxx
      docker push technbolts/jwtd:0.1.0

# Troubleshoots

      error: linker `cc` not found
      |
      = note: No such file or directory (os error 2)

      sudo apt install build-essential
