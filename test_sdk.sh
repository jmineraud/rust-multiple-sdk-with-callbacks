#!/bin/bash

echo "Starting the test"

cd rust-lib
cargo build --release

cd ../python-sdk
python3 ping_pong_sdk.py

cd ../node-sdk
if [ ! -d node_modules ]; then
    npm install
fi
npm start

echo "Completed the test"
