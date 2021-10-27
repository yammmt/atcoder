use proconio::input;

fn main() {
    input! {
        n: usize,
        xyn: [(isize, isize); n],
    }

    // 三点が同一直線上に位置してはならない
    // ゼロ割がサンプルにある優しい
    let mut ans = n * (n - 1) * (n - 2) / 6;
    for a in 0..n {
        for b in a + 1..n {
            for c in b + 1..n {
                if (xyn[b].1 - xyn[a].1) * (xyn[c].0 - xyn[a].0)
                    == (xyn[c].1 - xyn[a].1) * (xyn[b].0 - xyn[a].0)
                {
                    ans -= 1;
                }
            }
        }
    }

    println!("{}", ans);
}
