Bitcode compilation:
https://stackoverflow.com/questions/69042049/how-to-compile-rust-to-llvm-bitcode-including-dependencies

side module instructions:
https://medium.com/@arora.aashish/webassembly-dynamic-linking-1644c9f40c8c
https://emscripten.org/docs/compiling/Dynamic-Linking.html#runtime-dynamic-linking-with-dlopen

emscriptenfs:
https://emscripten.org/docs/porting/files/file_systems_overview.html

sqlite api macro emulation:
https://github.com/asg017/sqlite-loadable-rs/blob/main/src/ext.rs

```
RUSTFLAGS="--emit=llvm-bc" cargo build --target wasm32-unknown-unknown
```

```
RUSTFLAGS="--emit=llvm-bc" cargo build -Z build-std=panic_abort,std --target wasm32-unknown-unknown
```

as side module from rust using emscripten target:
https://www.hellorust.com/setup/emscripten/
https://github.com/rust-lang/rust/issues/98155

```
rustup target add wasm32-unknown-emscripten
link-arg=-sSIDE_MODULE=2 --target wasm32-unknown-emscripten --crate-type cdylib

RUSTFLAGS="-C link-arg=-sSIDE_MODULE=2" cargo build --target wasm32-unknown-emscripten

// release build that seems to work:
RUSTFLAGS="-C link-arg=-sSIDE_MODULE=2 -Zlink-native-libraries=no" cargo build --release --target wasm32-unknown-emscripten
// from: https://github.com/rust-lang/rust/pull/98303
```

Packing files into the fs:
https://emscripten.org/docs/porting/files/packaging_files.html#packaging-files
https://github.com/emscripten-core/emscripten/blob/main/tools/file_packager.py

~/workspace/emsdk/upstream/emscripten/tools/file_packager ../../js/example/public/test_runtime_ext.so --use-preload-plugins --preload ./target/wasm32-unknown-emscripten/release/test_runtime_ext.so --no-node > ../../js/example/public/load.js

```
'v': void type

'i': 32-bit integer type

'j': 64-bit integer type (currently does not exist in JavaScript)

'f': 32-bit float type

'd': 64-bit float type
```

https://github.com/emscripten-core/emscripten/issues/12268
-s ERROR_ON_UNDEFINED_SYMBOLS=0 ?

/_
if (!flags.allowUndefined) {
_/

does it work with asyncify?
