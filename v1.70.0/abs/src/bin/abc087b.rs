use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        x: usize,
    }

    let mut ans = 0;
    for i in 0..=a {
        for j in 0..=b {
            for k in 0..=c {
                let cur = 500 * i + 100 * j + 50 * k;
                if cur == x {
                    ans += 1;
                }
            }
        }
    }

    println!("{ans}");
}
