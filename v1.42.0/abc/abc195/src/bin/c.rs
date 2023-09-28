use proconio::input;

fn main() {
    input! {
        n: i64,
    }
    let mut ans = 0;
    let mut base = 1000;
    loop {
        if n < base {
            break;
        }

        ans += n - base + 1;
        base *= 1000;
    }
    println!("{}", ans);
}
