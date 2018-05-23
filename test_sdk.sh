#!/bin/bash

echo "Starting the test"

cd rust-lib
cargo build --release

cd ../python-sdk
python3 ping_pong_sdk.py

echo "Completed the test"
