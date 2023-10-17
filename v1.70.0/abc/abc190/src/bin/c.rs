use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        abm: [(usize, usize); m],
        k: usize,
        cdk: [(usize, usize); k],
    }

    let mut ans = 0;
    for i in 0..2u32.pow(k) {
        let mut balls = vec![false; n];
        let mut ii = i;
        for cd in &cdn {
            if i % 2 == 0 {
                balls[cd.0 - 1] = true;
            } else {
                balls[cd.1 - 1] = true;
            }
            ii /= 2;
        }

        let mut cur = 0;
        for ab in &abm {
            let a = ab.0 - 1;
            let b = ab.1 - 1;
            if balls[a] && balls[b] {
                cur += 1;
            }
        }
        ans = ans.max(cur);
    }

    println!("{ans}");
}
