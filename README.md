# Belajar Python — Index Utama

Background: web dev JS/TS yang sekarang mengerjakan project **Qt (C++)** dan **MOOS-DB**, belajar Python lagi dari awal. Gaya belajar mengikuti [cpp-learning-fasttrack](https://github.com/galihroswandi/cpp-learning-fasttrack): fase bertahap, contoh domain kapal/ECDIS, challenge & mini-project, serta catatan format **KONSEP → ATURAN → KODE MINIMAL → JALANKAN → BEDAH BARIS → EKSPERIMEN → CEK PAHAM**.

Roadmap + status lengkap: **[docs/belajar-python/README.md](docs/belajar-python/README.md)**. Setiap topik = 1 GitHub Issue ([daftar issue](https://github.com/galihroswandi/python-learning-fasttrack/issues)).

## Peta Fase

| Fase | Fokus | Topik | Folder |
|---|---|---|---|
| 1 | Fondasi Python | 1–11 | `fase-1-fondasi/` |
| 2 | Intermediate | 12–20 | `fase-2-intermediate/` |
| 3 | Advanced (I/O, Network, Concurrency) | 21–26 | `fase-3-advanced/` |
| 4 | Python + Qt (PySide6) | 27–33 | `fase-4-python-qt/` |
| 5 | MOOS-DB & pymoos | 34–40 | `fase-5-moosdb/` |
| 6 | Capstone | 41–42 | `fase-6-capstone/` |

## Cara kerja per topik
1. Buka issue topik → baca tujuan & checklist.
2. Kerjakan kode di folder `fase-N-.../NN-nama-topik/` (1 file **contoh** + 1 file **latihan**).
3. Tulis catatan di `docs/belajar-python/topik-NN-*.md` dari [template](docs/belajar-python/_template-topik.md).
4. Update status di tabel roadmap → commit `feat: <topik>` → close issue.

## Setup
```bash
python3 -m venv .venv && source .venv/bin/activate
```
Satu venv di root cukup untuk Fase 1-3; Fase 4-5 bisa tambah `pip install PySide6 pymoos`.

## Lain-lain
- Tips & jebakan: [docs/belajar-python/tips-umum.md](docs/belajar-python/tips-umum.md)
- Repo C++ pembanding (porting latihan ke Python di Fase 4): [cpp-learning-fasttrack](https://github.com/galihroswandi/cpp-learning-fasttrack)
