set windows-powershell := true

default:
  cargo run

build:
  cargo build

format:
  cargo fmt; cargo clippy;

test:
  cargo llvm-cov --ignore-filename-regex main
