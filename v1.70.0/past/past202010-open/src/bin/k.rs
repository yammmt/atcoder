use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

const MOD: usize = 1_000_000_000;
const A_MAX: usize = 20;

#[fastout]
fn main() {
    input! {
        k: usize,
    }
    let mut nk = vec![];
    let mut akn = vec![];
    for _ in 0..k {
        input! {
            n: usize,
            an: [Usize1; n],
        }
        nk.push(n);
        akn.push(an);
    }
    input! {
        q: usize,
        bq: [Usize1; q],
    }

    // a の範囲が [1, 20] でしかない
    // append 前の数字の出現数と append する数字の出現数及びその列内での転倒数とで
    // 和を取れば終わり, のはず.
    // ちょっと定数倍重い気もするが制限時間が 5 s

    let mut cnt_per_akn = vec![vec![0; A_MAX]; k];
    let mut inversion_num_cnt_per_akn = vec![0; k];
    for i in 0..k {
        for &a in &akn[i] {
            for j in a + 1..A_MAX {
                // これも MOD 取らんで良い気がするが
                inversion_num_cnt_per_akn[i] += cnt_per_akn[i][j];
                inversion_num_cnt_per_akn[i] %= MOD;
            }
            // これは高々 10^5 だから MOD 取らんで良い
            cnt_per_akn[i][a] += 1;
        }
    }

    let mut ans = 0;
    let mut cnt_x = vec![0; A_MAX];
    for b in bq {
        for i in 0..A_MAX {
            for j in i + 1..A_MAX {
                ans += cnt_x[j] * cnt_per_akn[b][i];
                ans %= MOD;
            }
        }
        ans += inversion_num_cnt_per_akn[b];
        ans %= MOD;

        for i in 0..A_MAX {
            cnt_x[i] += cnt_per_akn[b][i];
            cnt_x[i] %= MOD;
        }
    }

    println!("{ans}");
}
