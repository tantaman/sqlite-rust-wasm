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
