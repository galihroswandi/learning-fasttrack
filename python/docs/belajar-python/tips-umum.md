# Tips Umum — Python untuk Developer JS & C++

## Perbedaan mendasar

| JavaScript | C++ | Python |
|---|---|---|
| Tipe dinamis | Statis | Dinamis (+ type hints opsional) |
| `{ }` blok | `{ }` blok | **Indentasi** blok |
| `null` / `undefined` | `nullptr` | `None` |
| `===` | `==` | `==` (nilai), `is` (identitas) |
| Template literal | `QString::arg` | f-string |
| `npm` / `package.json` | CMake / vcpkg | `pip` + venv + `pyproject.toml` |
| GC | Manual / smart pointer | GC + reference counting |
| `async/await` | `QFuture` / thread | `asyncio` / `threading` |
| `console.log` | `qDebug()` | `logging` / `print` |

## Jebakan umum
1. **Mutable default argument**: `def f(x=[])` — list dibagi antar pemanggilan.
2. **Late binding lambda di loop**: gunakan `lambda i=i: ...`.
3. **GIL**: thread tidak mempercepat kerja CPU-bound.
4. **Import circular**: pecah modul atau import di dalam fungsi.
5. **Widget Qt disentuh dari thread lain** → crash/undefined. Pakai signal.
6. **Callback pymoos berjalan di thread MOOS**, bukan main thread.

## Command cepat
```bash
python3 -m venv .venv && source .venv/bin/activate
pip install -e .
ruff check . && ruff format .
pytest -q
```
