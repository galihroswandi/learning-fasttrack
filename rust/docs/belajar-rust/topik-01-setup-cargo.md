# Topik R01 — Setup: rustup, cargo & Project Pertama
Selesai: 2026-11-02 · Issue: #43

## 1. KONSEP
`cargo` adalah package manager (seperti npm) sekaligus build system (seperti CMake + Ninja) untuk Rust. Satu tool mengurus dependency, kompilasi ke binary, test, format, dan linting. Rust secepat C++ karena dikompilasi langsung ke kode mesin lewat LLVM (backend yang sama dengan clang), tanpa VM dan tanpa garbage collector.

### Analogi C++ / JavaScript
| C++ / JS | Rust |
|---|---|
| `package.json` | `Cargo.toml` |
| `package-lock.json` | `Cargo.lock` |
| `npm install <paket>` | `cargo add <crate>` |
| `npm run` / `npm test` | `cargo run` / `cargo test` |
| `build/` (CMake) | `target/` |
| eslint / prettier | `cargo clippy` / `cargo fmt` |
| npmjs.com | crates.io, docs.rs |

## 2. ATURAN
- Kapan dipakai:
  - `cargo check`: cek apakah kode bisa dikompilasi tanpa membuat binary (cepat, dipakai saat menulis kode).
  - `cargo build`: buat binary debug.
  - `cargo build --release`: buat binary teroptimasi, dipakai untuk production **dan** untuk mengukur kecepatan.
  - `cargo fmt` untuk merapikan format, `cargo clippy` untuk saran idiom.
- Kapan TIDAK dipakai:
  - Jangan menilai kecepatan dari `cargo run` biasa (build debug). Pada loop 1 miliar iterasi: debug ~9.1 detik vs release ~0.9 detik.
  - `cargo new` di dalam repo git yang sudah ada tidak membuat `.gitignore`, jadi `target/` harus di-ignore manual (`/target`).
- Common mistake / error compiler yang sering muncul:
  - Import tidak terpakai → warning `unused import`.
  - `let` tanpa `mut` lalu diubah → E0384.
  - Memakai ternary `? :` → Rust tidak punya; pakai `if ... else ...` sebagai ekspresi.

## 3. KODE MINIMAL
`src/main.rs` (dependency: `cargo add colored`)
```rust
use colored::Colorize;

fn main() {
    let nama = "KRI Contoh";
    let knot = 12.5;

    println!("{} bergerak {} knot", nama, knot);
    println!("Kapal: {}", nama.bold());
    let value_with_color = if knot < 10.0 {
        knot.to_string().green()
    } else {
        knot.to_string().red()
    };
    println!("Kecepatan: {} knot", value_with_color);
}
```

## 4. JALANKAN
```bash
cargo new hello_vessel && cd hello_vessel
cargo add colored
cargo run
```
Output:
```
KRI Contoh bergerak 12.5 knot
Kapal: KRI Contoh
Kecepatan: 12.5 knot
```
(`Kapal` tebal, kecepatan merah karena `knot >= 10.0`.)

Perbandingan debug vs release (`examples/bench_loop.rs`, 1 miliar iterasi):
```bash
cargo build && cargo build --release
time ./target/debug/hello_vessel     # ~9.1 detik
time ./target/release/hello_vessel   # ~0.9 detik
```

## 5. BEDAH BARIS
| Baris | Penjelasan |
|---|---|
| `use colored::Colorize;` | Meng-import *trait* yang menambah method `.bold()`, `.green()`, `.red()`. Tanpa ini error E0599 (method tidak dikenali). |
| `fn main() {` | Titik masuk program, seperti `int main()` di C++. |
| `let nama = "KRI Contoh";` | Variabel immutable (default). Mengubahnya → E0384. |
| `println!(...)` | Macro (tanda `!`), bukan fungsi: jumlah argumen bebas dan format string dicek saat kompilasi. |
| `{}` / `{nama}` | Placeholder. `{nama}` hanya menerima nama variabel, bukan pemanggilan method. |
| `if knot < 10.0 { ... } else { ... }` | `if` adalah ekspresi (pengganti ternary). Kedua cabang harus bertipe sama; tanpa `;` di baris terakhir cabang. |
| `knot.to_string().green()` | Ubah angka ke teks dulu, baru diberi warna. |

## 6. EKSPERIMEN
- Aku ubah `let kecepatan_knot` tanpa `mut` → error `E0384`: cannot assign twice to immutable variable → tambahkan `mut` (compiler menyarankannya di `help:`).
- Aku hapus `use colored::Colorize;` → error `E0599`: method `bold`/`green`/`red` tidak ditemukan → trait harus di-import dengan `use`.
- Aku tulis `println!("{} dan {}", 5);` → error: 2 posisi di format string tapi 1 argumen → jumlah `{}` harus sama dengan argumen (dicek saat kompilasi, beda dengan `printf` di C).
- Benchmark loop selesai dalam ~3 ms → optimizer menghapus loop (dihitung dengan rumus) → bungkus nilai dengan `std::hint::black_box` di dalam loop, hasilnya jadi ~0.9 detik.

## 7. CEK PAHAM
1. Beda `cargo check` dan `cargo build`?
   `check` memeriksa kode tanpa membuat binary (lebih cepat); `build` memeriksa dan menghasilkan binary.
2. Kenapa `Cargo.lock` di-commit untuk aplikasi?
   `Cargo.toml` hanya berisi rentang versi, sehingga tiap orang bisa mendapat versi dependency berbeda. `Cargo.lock` mengunci versi persis supaya build reproducible.
3. Apa fungsi `target/`?
   Menyimpan hasil build (binary dan file perantara, termasuk dependency yang sudah dikompilasi). Bukan padanan `node_modules` (itu mirip `~/.cargo/registry`); lebih mirip `build/` di CMake. Bisa dibuat ulang, jadi di-gitignore.
