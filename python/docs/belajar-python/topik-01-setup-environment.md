# Topik 01 — Setup Environment: Python, venv, pip, ruff
Selesai: 2026-10-25 · Issue: #1

## 1. KONSEP
Python yang terpasang di sistem (`/usr/bin/python3`) dipakai oleh OS. Kalau paket di-install langsung ke sana, paket bisa bentrok dan merusak tool sistem. Ubuntu 24.04 karena itu menolak `pip install` ke Python sistem dengan error `externally-managed-environment`.

**venv** (virtual environment) adalah folder Python terisolasi per project. Di dalamnya ada interpreter dan paket sendiri, terpisah dari sistem.

**ruff** adalah alat untuk memeriksa dan merapikan kode. `ruff check` mencari kesalahan (linter), `ruff format` merapikan gaya penulisan (formatter). Ruff hanya dipakai saat menulis kode, bukan saat program berjalan.

### Analogi JavaScript / C++
| JavaScript / C++ | Python |
|---|---|
| `package.json` | `requirements.txt` |
| `npm install` | `pip install -r requirements.txt` |
| `node_modules/` | `.venv/` |
| `npm` otomatis pakai `node_modules` | Harus `source .venv/bin/activate` dulu |
| ESLint + Prettier | `ruff check` + `ruff format` |

## 2. ATURAN
- Kapan dipakai: setiap project Python, sebelum install paket apa pun.
- Kapan TIDAK dipakai: jangan `pip install` ke Python sistem.
- Membuat ulang venv di mesin lain:
  ```bash
  python3 -m venv .venv
  source .venv/bin/activate
  pip install -r requirements.txt
  ```
- Cek venv aktif dengan `echo $VIRTUAL_ENV` atau `which python`. Prompt zsh bisa menyembunyikan `(.venv)`, jadi jangan hanya mengandalkan prompt.
- Common mistake:
  - Lupa aktivasi, sehingga paket terpasang di tempat yang salah.
  - Commit `.venv/`. Folder ini spesifik untuk mesin dan sudah ada di `.gitignore`. Yang di-commit adalah `requirements.txt`.

## 3. KODE MINIMAL
`fase-1-fondasi/01-setup-environment/contoh.py`
```python
import sys

print(sys.version)
print(sys.prefix)
```

## 4. JALANKAN
Di dalam venv:
```bash
source .venv/bin/activate
python fase-1-fondasi/01-setup-environment/contoh.py
```
Output:
```
3.12.3 (main, Aug 31 2026, 10:18:26) [GCC 13.3.0]
/home/galih/Documents/PersonalProject/learning-fasttrack/python/.venv
```

Di luar venv:
```bash
deactivate
/usr/bin/python3 fase-1-fondasi/01-setup-environment/contoh.py
```
Output:
```
3.12.3 (main, Aug 31 2026, 10:18:26) [GCC 13.3.0]
/usr
```

Cek ruff:
```bash
ruff check fase-1-fondasi/01-setup-environment/contoh.py
ruff format fase-1-fondasi/01-setup-environment/contoh.py
```

## 5. BEDAH BARIS
| Baris | Penjelasan |
|---|---|
| `import sys` | Mengimpor modul `sys`, yang berisi info tentang interpreter yang sedang berjalan. |
| `print(sys.version)` | Menampilkan versi Python. Di venv dan di sistem sama (3.12.3), karena venv dibuat dari Python itu. |
| `print(sys.prefix)` | Menampilkan folder tempat Python berada. Ini yang berbeda: `/usr` di sistem, `.venv` di venv. |

## 6. EKSPERIMEN
- `deactivate`, lalu `pip install requests`
  → Error `externally-managed-environment`. Pip menolak karena Python sistem dikelola OS. Solusinya membuat venv.
- Tambahkan `import os` yang tidak dipakai ke `contoh.py`, lalu jalankan `ruff check`
  → Muncul `F401` (`os` imported but unused) dan `I001` (urutan import). `ruff format` tidak menghapus `import os`, karena formatter hanya mengatur gaya.
  → `ruff check --fix` menghapus `import os` dan mengurutkan import.

## 7. CEK PAHAM
1. Apa bedanya `python3` sistem dan `python` di dalam venv?
   Python sistem dipakai OS. Python di venv punya paket sendiri yang terpisah, jadi paket yang di-install di project tidak memengaruhi OS.
2. Kenapa `.venv/` tidak di-commit, tapi `requirements.txt` di-commit?
   `.venv/` spesifik untuk mesin dan versi Python tertentu, path di dalamnya tidak cocok di komputer lain, dan ukurannya besar. `requirements.txt` kecil dan cukup untuk membuat ulang venv di mana pun.
3. Apa yang berubah saat `source .venv/bin/activate`?
   Variabel `PATH` diubah sehingga `python` dan `pip` menunjuk ke `.venv/bin` lebih dulu. Itu sebabnya `which python` berubah.
