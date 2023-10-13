use proconio::input;
fn main() {
    input! {
        n: usize,
        y: usize,
    }

    for i in 0..=n {
        for j in 0..=n - i {
            let k = n - i - j;
            let money = 10000 * i + 5000 * j + 1000 * k;
            if money == y {
                println!("{i} {j} {k}");
                return;
            }
        }
    }

    println!("-1 -1 -1");
}
