#!/usr/bin/env python3
"""Hitung progress belajar dari tabel status di roadmap tiap bahasa dan tulis ke README utama.

Satu baris tabel = satu item. ✅ = selesai (dihitung), 🔄 = sedang dikerjakan, ⏳ = belum.
Pakai: python3 scripts/progress.py
"""
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
TRACKS = [
    ("C++ & Qt", "cpp", "cpp/docs/belajar-qt/README.md"),
    ("Python", "python", "python/docs/belajar-python/README.md"),
    ("Rust", "rust", "rust/docs/belajar-rust/README.md"),
]
START, END = "<!-- PROGRESS:START -->", "<!-- PROGRESS:END -->"


def count(path):
    done = doing = todo = 0
    for line in (ROOT / path).read_text().splitlines():
        if not line.startswith("|"):
            continue
        cells = [c.strip() for c in line.strip().strip("|").split("|")]
        if len(cells) < 3:
            continue
        status = cells[2]
        if status == "✅":
            done += 1
        elif status == "🔄":
            doing += 1
        elif status == "⏳":
            todo += 1
    return done, doing, todo


def bar(pct, width=20):
    filled = round(pct / 100 * width)
    return "█" * filled + "░" * (width - filled)


rows = ["| Track | Progress | Selesai | Berjalan | Belum | Total |", "|---|---|--:|--:|--:|--:|"]
tot_d = tot_all = 0
for name, folder, path in TRACKS:
    d, g, t = count(path)
    total = d + g + t
    pct = 100 * d / total if total else 0
    tot_d += d
    tot_all += total
    rows.append(f"| [{name}]({folder}/README.md) | `{bar(pct)}` **{pct:.0f}%** | {d} | {g} | {t} | {total} |")
all_pct = 100 * tot_d / tot_all if tot_all else 0
rows.append(f"| **Semua** | `{bar(all_pct)}` **{all_pct:.0f}%** | {tot_d} | | | {tot_all} |")
block = f"{START}\n" + "\n".join(rows) + f"\n\n_Dihitung otomatis dari tabel status di roadmap tiap track (`python3 scripts/progress.py`). Hanya ✅ yang dihitung selesai._\n{END}"

readme = ROOT / "README.md"
text = readme.read_text()
text = re.sub(re.escape(START) + r".*?" + re.escape(END), lambda m: block, text, flags=re.S)
readme.write_text(text)
print("\n".join(rows))
