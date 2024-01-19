use proconio::fastout;
use proconio::input;
use proconio::marker::Bytes;

const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        k: Bytes,
        d: usize,
    }
    let n = k.len();

    let mod_add = |a: &mut usize, b| *a = (*a + b) % MOD;
    let mod_sub = |a: &mut usize, b| *a = (*a + MOD - b) % MOD;

    // dp[a][b]: 和のあまりが b で c=1 なら K 未満確定
    let mut dp = vec![vec![0; 2]; d];
    dp[0][0] = 1;
    // 上から i 桁目で加算元の総和が j で今の桁を l
    for i in 0..n {
        let mut dp_nxt = vec![vec![0; 2]; d];
        let cur_k = (k[i] - b'0') as usize;
        for j in 0..d {
            for is_less in 0..2 {
                for l in 0..10 {
                    let sum_next = (j + l) % d;
                    if l < cur_k {
                        mod_add(&mut dp_nxt[sum_next][1], dp[j][is_less]);
                    } else if l == cur_k {
                        // 今の値と k[i] が同じ => K 未満確定か否かは直前の状態を引き継ぐ
                        mod_add(&mut dp_nxt[sum_next][is_less], dp[j][is_less]);
                    } else if is_less == 1 {
                        // K 未満確定 => そのまま
                        mod_add(&mut dp_nxt[sum_next][1], dp[j][1]);
                    }
                }
            }
        }
        dp = dp_nxt;
    }

    // -1: 0 を弾く
    let mut ans = 0;
    mod_add(&mut ans, dp[0][0]);
    mod_add(&mut ans, dp[0][1]);
    mod_sub(&mut ans, 1);
    println!("{ans}");
}
