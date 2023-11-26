use proconio::fastout;
use proconio::input;

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        ak: [usize; k],
    }

    // 鐘が鳴った時間の差分が何の倍数か
    // すべての隣接二つの差の公約数の数
    // 順に約数列挙していくと, K * sqrt(A_max) の時間がかかる
    // 等間隔で N 回鳴ったので,  d は最小でも (A_max-A_min)/N

    // 累積 GCD 過去にあったな, 計算量は記憶と違うけれど
    // 累積 GCD を求めて約数列挙してなんとか
    let mut a_diff = vec![];
    for i in 1..k {
        a_diff.push(ak[i] - ak[i - 1]);
    }

    let mut all_gcd = a_diff[0];
    for i in 1..k - 1 {
        all_gcd = gcd(all_gcd, a_diff[i]);
    }
    // println!("{:?}", a_diff);
    // println!("gcd: {all_gcd}");

    let mut divisors = vec![];
    let mut d = 1;
    while d * d <= all_gcd {
        if all_gcd % d == 0 {
            divisors.push(d);
            if all_gcd / d != d {
                divisors.push(all_gcd / d);
            }
        }
        d += 1;
    }
    divisors.sort_unstable();
    // println!("{:?}", divisors);

    let mut ans = vec![];
    for d in divisors {
        if ak[0] + (n - 1) * d as usize >= ak[k - 1] {
            ans.push(d);
        }
    }

    println!("{}", ans.len());
    for (i, a) in ans.iter().enumerate() {
        print!("{a}");
        if i == ans.len() - 1 {
            println!();
        } else {
            print!(" ");
        }
    }
}
