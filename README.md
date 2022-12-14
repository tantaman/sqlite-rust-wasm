Statically link a SQLite extension written in Rust to a WASM build of SQLite.

- `js/example/` contains an example app that uses the built wasm
- `wa-sqlite/` is our wasm port of SQLite
- `rs/` contains our rust lib

# Building

```
cd wa-sqlite
make
```

This will build and link in the `Rust` code as well.

The interesting commit that updated `wa-sqlite` to include the `rust` code is this one:
https://github.com/rhashimoto/wa-sqlite/commit/22985f93bcff1b435a7355962812619ec89a3b20

# Running Example App

```
cd js
pnpm install
pnpm start
```

# Is it actually doing anything?

Yes. You can test that `testext_commit_hook` is being called by flipping the return value to 1.

In that case, the demo app fails because the transaction is aborted.

Flipping it back to 0 and recompiling and everything works as expected.

# How is it done?

The Rust code is [compiled to LLVM bitcode against a wasm-unknown-unknown target](https://github.com/rhashimoto/wa-sqlite/blob/22985f93bcff1b435a7355962812619ec89a3b20/Makefile#LL216).

```
RUSTFLAGS="--emit=llvm-bc" cargo build --target wasm32-unknown-unknown
```

This bitcode is then able to be [linked by emscripten to the broader SQLite WASM bundle](https://github.com/rhashimoto/wa-sqlite/blob/22985f93bcff1b435a7355962812619ec89a3b20/Makefile#L256).
