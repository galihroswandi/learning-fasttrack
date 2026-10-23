# Tips Umum — Rust untuk Developer C++ & JS

| JavaScript | C++ | Rust |
|---|---|---|
| GC | RAII / smart pointer | Ownership + `Drop` (tanpa GC) |
| `null` | `nullptr` | `Option<T>` |
| `try/catch` | exception | `Result<T,E>` + `?` |
| `const`/`let` | `const` opt-in | immutable default, `mut` opt-in |
| `class extends` | `virtual` | `trait` (tanpa inheritance) |
| `npm` | CMake/vcpkg | `cargo` |
| `async/await` | `QFuture` | `async` + runtime (tokio) |
| `console.log` | `qDebug()` | `println!` / `tracing` |

## Cara membaca error compiler
1. Baca baris pertama (`error[E0382]: borrow of moved value`) — kodenya bisa dicari: `rustc --explain E0382`.
2. Lihat panah `^^^` dan label "value moved here" / "borrowed here".
3. Compiler biasanya menyarankan perbaikan (`help:`). Pahami dulu, jangan asal `.clone()`.

## Jebakan umum
1. Asal `.clone()` agar compiler diam → pahami dulu kenapa error.
2. `unwrap()` di mana-mana → pakai `?`/`expect` bermakna.
3. `Rc<RefCell<T>>` di mana-mana → tanda desain perlu dipikir ulang.
4. Blocking di async → `spawn_blocking`.
5. Menyentuh widget Qt dari thread Rust → kirim data lewat channel/queued invoke.
6. Mengira `unsafe` mematikan semua pengecekan — hanya 5 kemampuan tertentu.

## Command cepat
```bash
cargo new nama && cd nama
cargo check && cargo clippy && cargo fmt
cargo test && cargo run --release
```
