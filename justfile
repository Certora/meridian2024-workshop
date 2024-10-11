
build:
    RUSTFLAGS="-C strip=none -C debuginfo=2" cargo build --target=wasm32-unknown-unknown --release --features cvt

wasm2wat :=  "wasm2wat"
wat2wasm := "wat2wasm"

wat: build
    {{wasm2wat}} target/wasm32-unknown-unknown/release/certora_meridian24_token.wasm --generate-names -o foo.wat
    {{wat2wasm}} foo.wat --debug-names -o bar.wasm
    {{wasm2wat}} bar.wasm -o certora_meridian24_token.wat
    rm foo.wat bar.wasm

clean:
    cargo clean

update:
    cargo update -p nondet
