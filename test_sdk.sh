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

cd ../rust-lib
cargo build --release --features "java"

cd ../java-sdk
./gradlew run -q

echo "Completed the test"
