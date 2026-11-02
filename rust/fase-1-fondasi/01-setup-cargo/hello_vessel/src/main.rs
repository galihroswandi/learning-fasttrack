use colored::Colorize;

fn main() {
    let nama = "KRI Contoh"; // sudah ada di baris 4, pakai ulang
    let knot = 12.5;
    let arah = "utara";

    println!("\n---------------------------------------");
    println!("{nama}"); // 0 argumen tambahan
    println!("Kapal: {}", nama); // 1 argumen
    println!("{} bergerak {} knot", nama, knot); // 2 argumen
    println!("{} {} {}", nama, knot, arah); // 3 argumen

    println!("\n---------------------------------------");
    println!("Kapal: {}", nama.bold());
    let value_with_color = if knot < 10.0 {
        knot.to_string().green()
    } else {
        knot.to_string().red()
    };
    println!("Kecepatan: {} knot", value_with_color);
}
