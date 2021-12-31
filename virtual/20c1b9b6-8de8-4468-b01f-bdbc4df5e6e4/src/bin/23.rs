use proconio::input;

fn main() {
    input! {
        n: usize,
        ln: [usize; n],
    }

    let mut ans = 0;
    for i in 0..n {
        let li = ln[i];
        for j in i + 1..n {
            let lj = ln[j];
            if lj == li {
                continue;
            }

            for k in j + 1..n {
                let lk = ln[k];
                if lk == li || lk == lj {
                    continue;
                }

                if li < lj + lk && lj < lk + li && lk < li + lj {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}
