use proconio::input;

fn bit_rows(n: u32) -> Vec<Vec<usize>> {
    let mut ret = vec![];
    for b in 0..2u64.pow(n) {
        let mut cur = vec![];
        for i in 0..n {
            if b & (1 << i) > 0 {
                cur.push(i as usize);
            }
        }
        ret.push(cur);
    }
    ret
}

#[allow(clippy::needless_range_loop)]
fn main() {
    input! {
        n: usize,
        xn: [u64; n],
    }
    let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
    let mut ans = std::u64::MAX / 2;
    let bit_row = bit_rows(primes.len() as u32);
    for br in &bit_row {
        let mut cur = 1;
        let mut used = vec![];
        for i in 0..primes.len() {
            if br.contains(&i) {
                cur *= primes[i];
                used.push(primes[i]);
            }
        }
        if cur == 1 {
            continue;
        }

        let mut pass = true;
        for x in &xn {
            let mut pass_two = false;
            // 互いに素ではない
            for u in &used {
                if *x % *u == 0 {
                    pass_two = true;
                    break;
                }
            }
            if !pass_two {
                pass = false;
                break;
            }
        }

        if pass {
            ans = ans.min(cur);
        }
    }

    println!("{}", ans);
}
