use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        x: usize,
    }

    let mut ans = 0;
    for aa in 0..a + 1 {
        for bb in 0..b + 1 {
            for cc in 0..c + 1 {
                if 500 * aa + 100 * bb + 50 * cc == x {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}
