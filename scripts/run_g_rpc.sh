#!/bin/bash
cd /home/florin/github/crab-talk || exit
cargo build -p crab-talk-server -p crab-talk-client
cargo run -p crab-talk-server &
sleep 2  # Optional delay to let server start
cargo run -p crab-talk-client
wait  # Wait for background process to finish
