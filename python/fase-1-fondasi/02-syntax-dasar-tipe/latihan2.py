from decimal import Decimal

harga: int = 18_000
jumlah: int = 3
total: int = harga * jumlah
ribuan: int = total // 1000
sisa: int = total % 1000
print("1. =======================")
print(f"Total Harga {total}")
print(f"Total Ribuan {ribuan}")
print(f"Sisa Pembagian {sisa}")
print("==========================\n")
print("2. =======================")
item: str = "Makan"
nominal: int | None = 0
kategori: str | None = "Food"
rekening: str | None = "BCA"
print(f"Item: {item}")
print(f"Nominal: {nominal}")
print(f"Kategori: {kategori}")
print(f"Rekening: {rekening}")
print("==========================\n")
print("3. =======================")
lengkap: bool
if (
    item is not None
    and nominal is not None
    and kategori is not None
    and rekening is not None
):
    lengkap = True
else:
    lengkap = False
print(f"Data belanja = {lengkap}")
print("==========================\n")
print("4. =======================")
if nominal:
    print("Ada nominal")
else:
    print("Tidak ada nominal")

if nominal is not None:
    print("Nominal tidak bernilai None")
else:
    print("Nominal bernilai None")
print("==========================\n")
print("5. =======================")
print([1, 2] == [1, 2])
print([1, 2] == [1, 2])
print("==========================\n")
print("6. =======================")
daftar = ["kopi", "roti"]
salinan = daftar
salinan.append("teh")
print(f"Daftar: {daftar}")
salinan = daftar.copy()
print(f"Salinan: {salinan}")
print(daftar == salinan)
print(daftar != salinan)
print(daftar is salinan)
print("==========================\n")
print("6. =======================")
a = Decimal("10.0001")
b = Decimal("10.0002")
toleransi = Decimal("0.00001")
hasilSelisih = abs(a - b)
print(f"Nilai a = {a}")
print(f"Nilai b = {b}")
print(f"Toleransi = {toleransi}")
if hasilSelisih <= toleransi:
    print(f"Hasil selisih {hasilSelisih} artinya Masuk Toleransi")
else:
    print(f"Hasil selisih {hasilSelisih} artinya Tidak masuk Toleransi")
print("==========================\n")
