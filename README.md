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
