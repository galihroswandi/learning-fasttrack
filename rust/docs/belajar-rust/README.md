# Progress Belajar Rust — Fase 1-6

Background: web dev (JS/TS) + C++/Qt + MOOS-DB. Rust dipelajari karena penasaran dan karena relevan: ownership ≈ RAII yang dicek compiler, FFI/`cxx` ke Qt C++, dan parser log MOOS yang cepat.

Format sama dengan jalur Python & [`cpp-learning-fasttrack`](https://github.com/galihroswandi/cpp-learning-fasttrack): **KONSEP → ATURAN → KODE MINIMAL → JALANKAN → BEDAH BARIS → EKSPERIMEN → CEK PAHAM**.
Satu topik = satu GitHub Issue (label `rust`) = satu cargo project di `rust/` = satu catatan. Template: [_template-topik.md](_template-topik.md).

> ⭐ = topik inti yang paling sering bikin stuck (ownership, borrowing, lifetimes, unsafe/FFI). Jangan buru-buru; ulangi eksperimennya.
> Prasyarat: sudah nyaman dengan C++ (pointer/RAII) dari repo C++ dan minimal Fase 1 Python.

**Legenda status:** ✅ selesai · 🔄 sedang dikerjakan · ⏳ belum mulai

## Rust Fase 1 — Fondasi & Ownership

| No | Topik | Status | Issue | Catatan |
|----|-------|--------|-------|---------|
| R01 | Setup: rustup, cargo & Project Pertama | ⏳ | #43 | — |
| R02 | Variabel, Tipe Dasar & Mutability | ⏳ | #44 | — |
| R03 | Control Flow & Fungsi | ⏳ | #45 | — |
| R04 | Ownership & Move ⭐ | ⏳ | #46 | — |
| R05 | Borrowing & References (`&` / `&mut`) ⭐ | ⏳ | #47 | — |
| R06 | Struct, Enum & Pattern Matching | ⏳ | #48 | — |
| R07 | Option, Result & Error Handling Dasar | ⏳ | #49 | — |
| R08 | Collections & String: Vec, HashMap, String | ⏳ | #50 | — |
| R09 | Modul, Crate & Visibility | ⏳ | #51 | — |
| R10 | 🎯 Challenge Fase 1: Fleet Manager CLI (Rust) | ⏳ | #52 | — |

## Rust Fase 2 — Intermediate

| No | Topik | Status | Issue | Catatan |
|----|-------|--------|-------|---------|
| R11 | Lifetimes ⭐ | ⏳ | #53 | — |
| R12 | Traits & Generics | ⏳ | #54 | — |
| R13 | Error Handling Lanjut: thiserror & anyhow | ⏳ | #55 | — |
| R14 | Iterator & Closure | ⏳ | #56 | — |
| R15 | Smart Pointers: Box, Rc, Arc, RefCell | ⏳ | #57 | — |
| R16 | Testing: unit, integration & doc test | ⏳ | #58 | — |
| R17 | File I/O, serde & Config | ⏳ | #59 | — |
| R18 | CLI (clap) & Logging (tracing) | ⏳ | #60 | — |
| R19 | 🎯 Challenge Fase 2: Log Analyzer (Rust) | ⏳ | #61 | — |

## Rust Fase 3 — Advanced (Concurrency, Network, FFI)

| No | Topik | Status | Issue | Catatan |
|----|-------|--------|-------|---------|
| R20 | Threads, Channel & Arc<Mutex> | ⏳ | #62 | — |
| R21 | async/await & tokio | ⏳ | #63 | — |
| R22 | Networking: TCP/UDP & HTTP | ⏳ | #64 | — |
| R23 | Database: SQLite (rusqlite/sqlx) | ⏳ | #65 | — |
| R24 | Unsafe & FFI ke C ⭐ | ⏳ | #66 | — |
| R25 | Workspace, Features & Performance | ⏳ | #67 | — |
| R26 | 🎯 Challenge Fase 3: Telemetry Server (Rust) | ⏳ | #68 | — |

## Rust Fase 4 — Rust + C++/Qt

| No | Topik | Status | Issue | Catatan |
|----|-------|--------|-------|---------|
| R27 | Rust ↔ C++ dengan `cxx` | ⏳ | #69 | — |
| R28 | Memakai Library Rust di Aplikasi Qt (C++) | ⏳ | #70 | — |
| R29 | cxx-qt: Qt dari Rust (Pengenalan) | ⏳ | #71 | — |
| R30 | 🎯 Mini-Project Fase 4: Geo Core Rust di Qt App | ⏳ | #72 | — |

## Rust Fase 5 — Rust + MOOS-DB

| No | Topik | Status | Issue | Catatan |
|----|-------|--------|-------|---------|
| R31 | MOOS dari Sudut Rust: Opsi Integrasi | ⏳ | #73 | — |
| R32 | Rust Client: Subscribe & Publish ke MOOSDB | ⏳ | #74 | — |
| R33 | Parser `.alog` Cepat di Rust | ⏳ | #75 | — |
| R34 | Rust MOOS Client → Qt UI (Thread-Safe) | ⏳ | #76 | — |
| R35 | 🎯 Challenge Fase 5: MOOS Monitor (Rust core + Qt UI) | ⏳ | #77 | — |

## Rust Fase 6 — Capstone

| No | Topik | Status | Issue | Catatan |
|----|-------|--------|-------|---------|
| R36 | Idiom Rust & Code Review Diri Sendiri | ⏳ | #78 | — |
| R37 | 🏁 Capstone: Vessel Analytics Engine (Rust) | ⏳ | #79 | — |

## Referensi
- [tips-umum.md](tips-umum.md)
- *The Rust Book* (doc.rust-lang.org/book), *Rust by Example*, `rustlings` — pilih salah satu sebagai pendamping.
