use proconio::input;

fn main() {
    input! {
        n: u64,
        m: u64
    }

    let mut g = m / n;
    while !(m % g == 0 && (m / g) >= n) {
        // println!("g: {}", g);
        g -= 1;
    }
    println!("{}", g);
}
