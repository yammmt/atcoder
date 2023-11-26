use proconio::fastout;
use proconio::input;
use std::collections::BTreeSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        an: [usize; n],
    }

    // 操作回数が 10^12 までいく可能性があり, 愚直シミュレーションは TLE
    // それぞれの子供に対して取得できる飴の数で二分探索とかできる？
    // 飴の数は最小値最大化で進んでいくので, 全員が b 個以上飴をもてるかを探して
    // 境目まで検索, 残りは愚直に頭から

    // fail の初期値結構ギリギリでは
    let mut pass = 0;
    let mut fail = usize::MAX / 2;

    while fail - pass > 1 {
        // println!("{fail} {pass}");
        let mid = (fail + pass) / 2;
        let mut cnt = 0;
        for a in &an {
            if *a >= mid {
                continue;
            }

            cnt += (mid - *a + k - 1) / k;
            if cnt > m {
                break;
            }
        }

        if cnt <= m {
            pass = mid;
        } else {
            fail = mid;
        }
    }

    // 愚直
    let mut m_left = m;
    let mut bts = BTreeSet::new();
    for (i, a) in an.iter().enumerate() {
        let mut aa = *a;
        if aa < pass {
            let m_used = (pass - *a + k - 1) / k;
            aa = a + k * m_used;
            m_left -= m_used;
        }
        bts.insert((aa, i));
    }

    for _ in 0..m_left {
        let Some(cur) = bts.pop_first() else { unreachable!(); };
        bts.insert((cur.0 + k, cur.1));
    }

    let mut ans = vec![0; n];
    while let Some(cur) = bts.pop_first() {
        ans[cur.1] = cur.0;
    }

    for (i, a) in ans.iter().enumerate() {
        print!("{a}");
        if i == n - 1 {
            println!();
        } else {
            print!(" ");
        }
    }
}
