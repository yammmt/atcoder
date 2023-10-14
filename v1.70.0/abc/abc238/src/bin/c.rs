use proconio::input;

const MOD_OF: usize = 998_244_353;
const REV2: usize = 499_122_177;

fn main() {
    input! {
        n: usize,
    }

    let mut n_digit_max = 0;
    let mut nn = n;
    while nn > 0 {
        n_digit_max += 1;
        nn /= 10;
    }

    let mut ans = 0usize;
    let mut n_min = 1;
    // i+1 桁目
    for _i in 0..n_digit_max {
        let n_max = if n_min * 10 > n { n } else { n_min * 10 - 1 };
        let n_len = n_max - n_min + 1;
        let a0 = 1;
        let an = a0 + n_len - 1;
        ans = (ans + ((((a0 + an) % MOD_OF) * (n_len % MOD_OF)) % MOD_OF * REV2)) % MOD_OF;

        n_min *= 10;
    }

    println!("{ans}");
}
