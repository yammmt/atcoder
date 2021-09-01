// :fu: :fu: :fu: 21-09 場合分けするタイプの数問 最悪
// WA: 制約を読むと最大で 10^9 回繰り返される

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        mut an: [i64; n],
    }

    an.sort();
    an.reverse();
    an.push(0);

    // 降順に並べると今見ている項より前に登場する項はすべて同じ値 (自身 + 1) となっている
    // 算数で工夫しないと K 減らして探索していってしまって TLE
    let mut ans = 0;
    let mut cur_k = 0;
    // 同じ値が i 個続く
    for i in 1..n + 1 {
        if an[i] == an[i - 1] {
            continue;
        }
        // println!("i: {}, could_use: {}", i, k - cur_k);

        let a_end = an[i - 1];
        if k - cur_k > (an[i - 1] - an[i] + 1) * i as i64 {
            // 全部揃えるまで消す
            let a_begin = an[i] + 1;
            let kousu = a_end - a_begin + 1;
            let span_sum = (a_begin + a_end) * kousu / 2;
            ans += span_sum * i as i64;
            cur_k += kousu * i as i64;
        } else {
            // 端数
            // d: a_end からカウントダウンした最小値
            let d = (k - cur_k) / i as i64;
            let m = (k - cur_k) % i as i64;
            // println!("{} {}", d, m);
            let a_begin = a_end - d + 1;
            // println!("[{}, {}]", a_begin, a_end);
            ans += ((a_begin + a_end) * (a_end - a_begin + 1) / 2) * i as i64;
            // println!("    {}", ans);
            ans += (an[i - 1] - d) * m;
            // println!("    {}", ans);
            cur_k = k;
        }

        // println!("  {}", ans);
        if cur_k == k {
            break;
        }
    }

    println!("{}", ans);
}
