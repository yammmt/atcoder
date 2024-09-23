use proconio::fastout;
use proconio::input;

// 領域外判定を端折るため +1 した
const A_MAX: usize = 1_000_002;

#[fastout]
fn main() {
    input! {
        n: usize,
        abn: [(usize, usize); n],
    }

    // イベントソートする or いもす法やるだけっぽくも見える

    let mut imos = vec![0; A_MAX];
    for (a, b) in abn {
        imos[a] += 1;
        imos[b + 1] -= 1;
    }

    let mut ans = 0;
    let mut cur = 0;
    for i in imos {
        cur += i;
        ans = ans.max(cur);
    }

    println!("{ans}");
}
