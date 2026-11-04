// const MAX_SPEED_KNOT: f32 = 30.0;
// const DETIK_PER_JAM: u32 = 60 * 60;

fn main() {
    // let nama = "KRI Contoh";
    // let mut speed = 12.5;
    // println!("{nama}: {speed} knot (maks {MAX_SPEED_KNOT})");

    // speed = 18.0;
    // println!("{nama}: {speed} knot (maks {MAX_SPEED_KNOT})");

    // println!("\n------------------------------CARA MENENTUKAN TIPE------------------------------");
    // println!("# CARA 1");
    // println!("- Rust memilih default: bilangan bulat jadi i32, desimal jadi f64");
    // let a = 5; // i32
    // let b = 2.5; // f64
    // println!("nilai a: {a}, nilai b: {b}");

    // println!("#CARA 2");
    // println!("- tulis tipe setelah titik dua:");
    // let c: u8 = 200;
    // println!("nilai c: {c}");

    // println!("#CARA 3");
    // println!("- tulis tipe sebagai akhiran angka");
    // let d = 5_u64;
    // println!("nilai d: {d}");

    // // ===== TUGAS 1 ====
    // let a = 5;
    // let b = 2.5;
    // let c: u8 = 200; // Tugas 2: experimen 1: rusak menjadi out of range 300
    // let d = 5_u64;
    // let e = 'K'; // char: kutip tanggal
    // let f = true;
    // // let g: u32 = -5; // Tugas 2: experimen 2: rusak unsigned tidak bertanda pake tanda minus

    // println!("{a} {b} {c} {d} {e} {f}");
    // println!("ukuran i32: {} byte", std::mem::size_of::<i32>());
    // println!("ukuran char: {} byte", std::mem::size_of::<char>());
    // println!("ukuran usize: {} byte", std::mem::size_of::<usize>());

    // Langkah 3
    // let speed = "12.5";
    // println!("speed teks: {speed}");

    // let speed: f64 = speed.parse().unwrap();
    // println!("speed angka: {speed}");

    // let speed = speed * 2.0;
    // println!("speed dikali 2: {speed}");

    // {
    //     let speed = 0.0;
    //     println!("dalam blok: {speed}");
    // }

    // println!("luar blok: {speed}");

    // Langkah 4 const
    // let speed = 15.0;
    // let persen = speed / MAX_SPEED_KNOT * 100.0;

    // // MAX_SPEED_KNOT = 40.0;
    // // let x = 5;
    // // const Y: i32 = x;

    // println!("maks: {MAX_SPEED_KNOT} knot");
    // println!("speed {speed} knot = {persen}% dari maksimum");
    // println!("satu jam = {DETIK_PER_JAM} detik");

    // Langkah 5
    let kapal = ("KRI Contoh", 12.5, true);
    let (nama, speed, aktif) = kapal;
    println!("{nama} {speed} {aktif}");
    println!("nama: {}", kapal.0);

    let speeds = [10, 12, 15];
    println!("{:#?}", speeds);
    println!("pertama: {}", speeds[0]);
    println!("jumlah: {}", speeds.len());
    // println!("{}", speeds[5]);
    // let speeds = [10, 12.5, 15];
    // println!("{:?}", speeds);
    println!("{}", kapal.3);

    let nol = [0; 4];
    println!("{:?}", nol);
}
