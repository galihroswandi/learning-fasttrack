use std::hint::black_box;

fn main() {
    let n = black_box(1_000_000_000u64);
    let mut total: u64 = 0;
    for i in 0..n {
        total = total.wrapping_add(black_box(i).wrapping_mul(i));
    }
    println!("total: {total}");
}
