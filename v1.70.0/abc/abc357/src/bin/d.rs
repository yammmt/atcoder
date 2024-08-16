use ac_library::modint::ModInt998244353 as Mint;
use proconio::fastout;
use proconio::input;

const MOD: u32 = 998_244_353;

#[fastout]
fn main() {
    input! {
        n: u64,
    }

    // 割る数が大きいのでループ検出しようにも 10^9 個の要素が出現し得るのでだめ

    let mut nn = n;
    let mut n_digits = vec![];
    while nn > 0 {
        n_digits.push(nn % 10);
        nn /= 10;
    }
    let k = n_digits.len() as u64;

    // 等比数列の和の公式より, k を N の桁数として N((10^k)^N - 1) / (10^k-1)
    //   == N(10^kN - 1) / (10^k-1)
    // 剰余算ライブラリに頼らないなら (a/b) MOD m = (a * b^-1) MOD m として求める

    let divided = Mint::new(n) * (Mint::new(10).pow(k).pow(n) - 1) / (Mint::new(10).pow(k) - 1);
    println!("{}", divided.val() % MOD);
}
