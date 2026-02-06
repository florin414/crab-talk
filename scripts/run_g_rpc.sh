#!/bin/bash
cd /home/florin/github/crab-talk || exit
cargo build
cargo run --bin server &
sleep 2  # Optional delay to let server start
cargo run --bin client
wait  # Wait for background process to finish

