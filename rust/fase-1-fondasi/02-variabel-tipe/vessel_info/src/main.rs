const MAX_SPEED_KNOT: f64 = 30.0;

fn main() {
    let vessel_data = ("KRI Contoh", 12.5, true);
    let speeds: [f64; 4] = [10.0, 12.5, 15.0, 20.0];
    let (nama, speed, status) = vessel_data;
    let kecepatan_baru = "18";
    let kecepatan_baru: f64 = kecepatan_baru.parse().unwrap();
    let speed_percentage = kecepatan_baru / MAX_SPEED_KNOT * 100.0;

    println!("Kapal: {nama} (aktif: {status})");
    println!("Kecepatan sekarang: {speed} knot");
    println!("Kecepatan baru: {kecepatan_baru} knot ({speed_percentage}% dari maksimum)");
    println!(
        "Pembacaan: Pertama {}, terakhir {}, jumlah {}",
        speeds[0],
        speeds[speeds.len() - 1],
        speeds.len()
    );
}
