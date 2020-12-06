use proconio::input;

fn main() {
    input! {
        n: usize,
        y: u64,
    }

    for a in 0..n + 1 {
        for b in 0..n + 1 {
            if a + b > n {
                break;
            }

            let c = n - a - b;
            if (a * 10000 + b * 5000 + c * 1000) as u64 == y {
                println!("{} {} {}", a, b, c);
                return;
            }
        }
    }
    println!("-1 -1 -1");
}
