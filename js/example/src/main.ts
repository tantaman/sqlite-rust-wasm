import * as SQLite from "wa-sqlite";
import SQLiteAsyncESMFactory from "wa-sqlite/dist/wa-sqlite-async.mjs";
// @ts-ignore
import { IDBBatchAtomicVFS } from "wa-sqlite/src/examples/IDBBatchAtomicVFS.js";
// @ts-ignore
import wasmUrl from "wa-sqlite/dist/wa-sqlite-async.wasm?url";
import { tag } from "./tag";

const module = await SQLiteAsyncESMFactory({
  locateFile(file: string) {
    return wasmUrl;
  },
});
const sqlite3 = SQLite.Factory(module);
sqlite3.vfs_register(
  new IDBBatchAtomicVFS("idb-batch-atomic", { durability: "relaxed" })
);

const filename = null;
const db = await sqlite3.open_v2(
  filename || ":memory:",
  SQLite.SQLITE_OPEN_CREATE |
    SQLite.SQLITE_OPEN_READWRITE |
    SQLite.SQLITE_OPEN_URI,
  filename != null ? "idb-batch-atomic" : undefined
);

const sql = tag(sqlite3, db);

console.log(await sql`BEGIN`);
console.log(await sql`CREATE TABLE foo (a)`);
console.log(await sql`INSERT INTO foo VALUES (1)`);
console.log(await sql`COMMIT`);

console.log(await sql`SELECT * FROM foo`);
