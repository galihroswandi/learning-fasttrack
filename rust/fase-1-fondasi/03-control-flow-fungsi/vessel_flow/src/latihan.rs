const MAX_SPEED_KNOT: f64 = 30.0;

fn status(knot: f64) -> &'static str {
    if knot > MAX_SPEED_KNOT {
        return "berbahaya";
    } else if knot > 10.0 {
        return "cepat";
    }

    "lambat"
}

fn persen(knot: f64) -> f64 {
    knot / MAX_SPEED_KNOT * 100.0
}

fn lapor(nama: &str, knot: f64) {
    println!(
        "{nama}, status {}, kecepatan ({}%)",
        status(knot),
        persen(knot)
    );
}

pub fn main() {
    let speeds = [5.0, 12.5, 25.0, 35.0];
    for s in speeds {
        lapor("KRI Contoh", s);
    }
}
