#!/bin/bash
set -e

RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/rust_template.wasm ./res/

near login //第一次需要登录，之后合约修改，不需要此步骤
near deploy --wasmFile target/wasm32-unknown-unknown/release/rust_template.wasm --accountId putong333.testnet

near call putong333.testnet get_puzzle_number  --accountId putong333.testnet
//注意cmd下需要将“转义，加\
near call putong333.testnet guess_solution '{\"solution\":\"88\"}'  --accountId putong333.testnet  

near call putong333.testnet set_solution '{\"solution\":\"33\"}'  --accountId putong333.testnet

cargo test data_input::tests::test_guess_solution -- --exact