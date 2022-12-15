.PHONY: sidemodule_bc
sidemodule_bc:
	emcc ../target/wasm32-unknown-unknown/debug/deps/test_runtime_ext.bc \
		-s EXPORTED_FUNCTIONS=_sqlite3_test_runtime_ext_init,_testext_fn \
		-s SIDE_MODULE=2 -o test_runtime_ext.wasm

.PHONY: sidemodule
sidemodule:
	emcc ../target/wasm32-unknown-unknown/debug/test_runtime_ext.wasm \
		-s EXPORTED_FUNCTIONS=_sqlite3_test_runtime_ext_init,_testext_fn \
		-s SIDE_MODULE=2 -o test_runtime_ext.wasm

