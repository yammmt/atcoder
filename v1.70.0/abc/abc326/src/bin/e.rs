use proconio::fastout;
use proconio::input;

static MOD: u64 = 998_244_353;

// n^p mod m (繰り返し二乗法)
fn repeat_square(n: u64, p: u64, m: u64) -> u64 {
    if p == 0 {
        1
    } else if p == 1 {
        n % m
    } else if p % 2 == 0 {
        repeat_square(n, p / 2, m).pow(2) % m
    } else {
        (n * repeat_square(n, p - 1, m)) % m
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        an: [u64; n],
    }

    // WA 方針: ダイスを i 回振って得る期待値
    //   1 回目: 必ず発生し各要素が確率 1/N で選択
    //   2 回目: 確率 (N-1)/N で発生, a[0] 以外は確率 1/(N-1) で選択
    //   3 回目: 確率 (N-1)/N * (N-2)/N で発生, a[0], a[1] 以外は確率 1/(N-2) で選択
    // ... とはならず WA となる. 例えば, N=3 として 2 回目に発生し得る数字は
    //   1 回目が 1 => 2, 3
    //   2 回目が 2 => 3
    // となり, 重複して 3 が発生する. こうなると累積和の式に落とし込めない.

    // an[0] が選ばれる確率: 1/N
    // an[1] が選ばれる確率: 1/N + (1/N)^2
    //     初回に選ばれる + 二回目に選ばれる (初手 an[0])
    // an[2] が選ばれる確率: 1/N + (1/N)^2 + (1/N)^3
    //     初回に選ばれる + 二回目に選ばれる + 三回目に選ばれる
    let n_inv = repeat_square(n as u64, MOD - 2, MOD);
    let mut prob = n_inv;
    let mut cur = 0;
    for a in an {
        cur = (cur + prob * a) % MOD;
        // prob * n_inv: an[i] を選んだ後に an[i+1] が選ばれる確率
        prob = (prob + prob * n_inv) % MOD;
    }

    println!("{cur}");
}
