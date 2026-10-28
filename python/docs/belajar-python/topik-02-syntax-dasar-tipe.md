# Topik 02 — Syntax Dasar, Tipe Dinamis & Type Hints
Selesai: 2026-10-28 · Issue: #2

## 1. KONSEP
Di Python, **tipe menempel pada nilai, bukan pada variabel**. `x = 5` lalu `x = "lima"` sah dijalankan. Type hint (`x: int = 5`) hanya catatan untuk pembaca dan alat bantu. Python tidak memeriksanya saat program berjalan. Yang memeriksanya adalah alat seperti mypy atau editor (Pylance), sebelum program dijalankan.

Blok kode dibuat dengan **indentasi 4 spasi**, bukan `{ }`. Baris yang berakhir `:` membuka blok.

Konsep yang paling sering menimbulkan bug: `None` (belum ada nilai) berbeda dari `0` (ada nilai, yaitu nol), dan `=` tidak menyalin isi, hanya menambah nama baru untuk objek yang sama.

### Analogi JavaScript / C++
| JavaScript / C++ | Python |
|---|---|
| `let x: number = 1` | `x: float = 1.0` (hint, tidak dienforce) |
| `double lat = -6.2;` (statis) | `lat = -6.2` (dinamis, tipe ikut nilai) |
| `null` / `undefined` | `None` |
| `===` | `==` (isi sama), `is` (objek sama) |
| `&&` `\|\|` `!` | `and` `or` `not` |
| `Math.floor(a / b)` | `a // b` |
| `{ }` untuk blok | indentasi 4 spasi |
| `[]` dan `{}` truthy | `[]` dan `{}` **falsy** |

## 2. ATURAN
- Kapan dipakai: type hint di semua variabel, parameter, dan return value agar kode mudah dibaca dan bisa dicek.
- `==` untuk membandingkan isi. `is` **hanya** untuk `None` (`x is None`), karena `None` hanya ada satu di seluruh program.
- Falsy: `False`, `None`, `0`, `0.0`, `""`, `[]`, `()`, `{}`, `set()`. Selain itu truthy, termasuk `"0"` dan `[0]`.
- Untuk memeriksa "belum terisi", pakai `is None`, bukan `if x:`. Nominal `0` itu falsy tapi sah.
- Immutable: `int`, `float`, `str`, `bool`, `tuple`. Mutable: `list`, `dict`, `set`.
- Uang jangan disimpan sebagai `float`. Pakai `int` (rupiah) atau `Decimal`.
- Common mistake:
  - Memakai `is` untuk angka atau string (Python memberi `SyntaxWarning`).
  - `salinan = daftar` dikira menyalin. Yang benar `daftar.copy()`.
  - Tidak memberi nilai pada semua cabang `if`, sehingga variabel tidak terdefinisi (`NameError`).
  - Tanda `;` di akhir baris tidak diperlukan.

## 3. KODE MINIMAL
`fase-1-fondasi/02-syntax-dasar-tipe/contoh.py` (ringkas):
```python
x: int = 0
y: float = 0.0
z: str = ""
mybool: bool = True
isNone: None = None

print(type(x))
print(isinstance(x, int))
print(isNone is None)
```

Draft transaksi flowvest, `latihan2.py`:
```python
nominal: int | None = None
kategori: str | None = "Food"
rekening: str | None = "BCA"

lengkap: bool
if item is not None and nominal is not None and kategori is not None and rekening is not None:
    lengkap = True
else:
    lengkap = False
```

## 4. JALANKAN
```bash
python fase-1-fondasi/02-syntax-dasar-tipe/contoh.py
```
Output:
```
<class 'int'>
True
<class 'float'>
True
<class 'str'>
True
<class 'bool'>
True
<class 'NoneType'>
True
```

```bash
python fase-1-fondasi/02-syntax-dasar-tipe/latihan.py
```
Output:
```
Nama Kapal: KRI Bima Suci
Koordinat Kapal: (-6.2, 106.816666)
Kecepatan Kapal: 100.5 knots (186.13 km/h)
```

## 5. BEDAH BARIS
| Baris | Penjelasan |
|---|---|
| `x: int = 0` | Variabel `x` bernilai 0 dengan type hint `int`. Hint tidak dicek oleh Python saat berjalan. |
| `type(x)` / `isinstance(x, int)` | `type` menampilkan tipe nilai, `isinstance` memeriksa apakah nilai bertipe tertentu. |
| `isNone is None` | Cara memeriksa `None`. Ini satu-satunya kasus `is` dipakai. |
| `nominal: int \| None` | Nominal boleh berupa `int` atau `None` (belum terisi). |
| `nominal is not None` | Memeriksa "sudah terisi". `0` dianggap terisi. |
| `total // 1000` / `total % 1000` | Pembagian bulat dan sisa pembagian, dipakai untuk memecah rupiah menjadi ribuan. |
| `f"{kmh:.2f}"` | f-string dengan dua angka di belakang koma. |

## 6. EKSPERIMEN
- Mengubah `rekening` dari `None` jadi `"BCA"` mengubah `lengkap` dari `False` jadi `True`, asalkan field lain terisi.
- Awalnya kondisi memakai `nominal != 0`. Saat `nominal = None`, `None != 0` bernilai `True`, sehingga `lengkap` tetap `True`. Setelah diganti `nominal is not None`, hasilnya benar (`False`).
- Versi dengan `if` bersarang `if(nominal != 0)` membuat `lengkap` tidak pernah diisi saat `nominal = 0`, lalu `print(lengkap)` error `NameError`.
- Pengecekan falsy (`if item and nominal and ...`) menganggap `nominal = 0` sebagai kosong. Itu menyesatkan, karena `0` adalah nilai yang sah.
- `[1, 2] == [1, 2]` bernilai `True`, tapi `[1, 2] is [1, 2]` bernilai `False`: isinya sama, objeknya berbeda.
- `salinan = daftar` lalu `salinan.append("teh")` ikut mengubah `daftar` (satu objek, dua nama). `daftar.copy()` membuat list baru, jadi `daftar` tidak berubah.
- `b = a` lalu `b = b + [3]` membuat list baru `[1, 2, 3]`, sedangkan `a` tetap `[1, 2]`. `b.append(3)` justru mengubah list yang sama dan `a` ikut berubah.
- Perbandingan float dengan `Decimal` dan toleransi: `abs(a - b) <= toleransi`.

## 7. CEK PAHAM
1. Apa bedanya `==` dan `is`?
   `==` membandingkan **isi nilai**. `is` membandingkan **identitas objek** (apakah objek yang sama di memori). `is` hanya dipakai untuk `None`.
2. Kenapa `0.1 + 0.2 != 0.3`?
   Float disimpan sebagai pecahan biner dengan presisi terbatas. `0.1` dan `0.2` tidak bisa direpresentasikan persis, sehingga jumlahnya meleset sedikit. Untuk uang pakai `int` atau `Decimal`; untuk membandingkan float pakai toleransi.
3. Type hint dicek siapa kalau bukan Python?
   Alat bantu seperti mypy dan editor (Pylance), saat analisis kode. Saat program berjalan, penanganan `None` tetap tugas kode, misalnya `if x is None`.
4. Kenapa `nominal = 0` tidak boleh disamakan dengan `nominal = None` di parser transaksi?
   `None` berarti belum diketahui (parser harus bertanya ke user), `0` berarti sudah diketahui dan nilainya nol. Pengecekan falsy menyamakan keduanya.
5. Kenapa `daftar` ikut berubah pada `salinan = daftar`?
   `=` hanya membuat nama baru untuk objek yang sama, jadi perubahan lewat satu nama terlihat di nama lain. `.copy()` membuat objek baru.
