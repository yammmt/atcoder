use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [u64; n],
        x: u64,
    }

    let asum = an.iter().sum::<u64>();
    let mut ans = (x / asum) * n as u64;
    let mut cur = asum * (ans / n as u64);
    let mut idx = 0;
    // println!("{}", cur);
    while cur <= x {
        cur += an[idx];
        ans += 1;
        if cur > x {
            println!("{}", ans);
            return;
        }

        idx += 1;
    }
}
