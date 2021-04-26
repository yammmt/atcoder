use proconio::input;

fn main() {
    input! {
        n: i64,
        a: i64,
        b: i64,
        c: i64,
    }

    let mut ans = std::i64::MAX / 2;
    for i in 0..10000 {
        if a * i > n {
            break;
        }

        for j in 0..10000 {
            let ij_yen = a * i + b * j;
            if ij_yen > n {
                break;
            }

            let k_yen = n - ij_yen;
            if k_yen % c == 0 {
                let k = k_yen / c;
                ans = ans.min(i + j + k);
            }
        }
    }
    println!("{}", ans);
}
