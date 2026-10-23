# Learning Fast Track

Repo belajar pribadi: web developer (JS/TS) yang sekarang mengerjakan project **Qt (C++)** dan **MOOS-DB**. Tiga track dengan format belajar yang sama: **KONSEP → ATURAN → KODE MINIMAL → JALANKAN → BEDAH BARIS → EKSPERIMEN → CEK PAHAM**, contoh domain kapal/ECDIS, fase bertahap, challenge, dan mini-project.

## 📊 Progress

<!-- PROGRESS:START -->
| Track | Progress | Selesai | Berjalan | Belum | Total |
|---|---|--:|--:|--:|--:|
| [C++ & Qt](cpp/README.md) | `████████████░░░░░░░░` **58%** | 30 | 1 | 21 | 52 |
| [Python](python/README.md) | `░░░░░░░░░░░░░░░░░░░░` **0%** | 0 | 0 | 42 | 42 |
| [Rust](rust/README.md) | `░░░░░░░░░░░░░░░░░░░░` **0%** | 0 | 0 | 37 | 37 |
| **Semua** | `█████░░░░░░░░░░░░░░░` **23%** | 30 | | | 131 |

_Dihitung otomatis dari tabel status di roadmap tiap track (`python3 scripts/progress.py`). Hanya ✅ yang dihitung selesai._
<!-- PROGRESS:END -->

## Track

| Track | Isi | Index | Pelacakan |
|---|---|---|---|
| **C++ & Qt** | C++ dasar → Qt Widgets → custom widget, DB, i18n, capstone (diimpor dari `cpp-learning-fasttrack`, history dipertahankan) | [cpp/](cpp/README.md) | Tabel status di roadmap |
| **Python** | Fondasi → PySide6 → MOOS-DB/pymoos → capstone | [python/](python/README.md) | Issue #1–#42 |
| **Rust** 🦀 | Ownership → concurrency/FFI → Rust + Qt/MOOS → capstone | [rust/](rust/README.md) | Issue #43–#79 |

## Cara kerja per topik
1. Buka issue topik (Python/Rust) atau roadmap (C++) → baca tujuan & checklist.
2. Kerjakan kode di folder `<track>/fase-N-.../NN-nama-topik/`.
3. Tulis catatan di `<track>/docs/belajar-*/topik-NN-*.md` dari template track.
4. Ubah status di roadmap track (⏳ → 🔄 → ✅), jalankan `python3 scripts/progress.py`, commit, close issue.

## Struktur
```
cpp/      fase-1..6 + docs/belajar-qt
python/   fase-1..6 + docs/belajar-python
rust/     fase-1..6 + docs/belajar-rust
scripts/  progress.py (hitung persentase)
```
