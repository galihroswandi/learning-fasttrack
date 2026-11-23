mod latihan;

fn main() {
    //## COBA SENDIRI
    // let knot = 12.5;
    // if knot > 20.0 {
    //     println!("sangat cepat");
    // } else if knot > 10.0 {
    //     println!("cepat");
    // } else {
    //     println!("lambat");
    // }

    // let status = if knot > 10.0 { "cepat" } else { "lambat" };
    // println!("{status}");

    //## Tugas 1
    // let knot = 12.5;
    // if knot > 20.0 {
    //     println!("sangat cepat");
    // } else if knot > 10.0 {
    //     println!("cepat");
    // } else {
    //     println!("lambat");
    // }

    // let status = if knot > 10.0 { "cepat" } else { "lambat" };
    // println!("status: {status}");

    //## Tugas 2
    // if knot {
    //     println!("x");
    //

    //## Langkah 3
    // let mut hitung = 3;
    // while hitung > 0 {
    //     println!("mundur: {hitung}");
    //     hitung -= 1;
    // }

    // for i in 0..3 {
    //     println!("putaran {i}");
    // }
    // for i in 0..=3 {
    //     println!("termasuk {i}");
    // }

    // let speeds = [10.0, 12.5, 15.0];
    // for s in speeds {
    //     println!("{s} knot");
    // }

    // let mut n = 0;
    // loop {
    //     n += 1;
    //     if n == 3 {
    //         break;
    //     }
    // }
    // println!("berhbenti di {n}");

    //## Langkah 4 Fungsi
    lapor("KRI Contoh", 12.5);
    let hasil = kali_dua(12.5);
    println!("{hasil}");
    println!("{}", status(12.5));
    println!("{}", status(5.0));

    latihan::main();
}

fn lapor(nama: &str, knot: f64) {
    println!("{nama} melaju {knot} knot");
}

fn kali_dua(x: f64) -> f64 {
    return x * 2.0;
}

fn status(knot: f64) -> &'static str {
    if knot > 10.0 {
        return "cepat";
    }
    "lambat"
}
