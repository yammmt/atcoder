// :fu: 21-06 半分全列挙やるだけの水色典型
// bitDP は制約上不可で 40C20 の全列挙も明らかに TLE

// 40min (WA: 25min)
// WA: v_first の範囲外参照 (RE) と lower_bound の境界

use proconio::input;

fn lower_bound(v: &[usize], k: usize) -> usize {
    let mut low: isize = -1;
    let mut high = v.len() as isize;

    while high - low > 1 {
        let mid = (low + high) / 2;
        if v[mid as usize] >= k {
            high = mid;
        } else {
            low = mid;
        }
    }
    high as usize
}

fn main() {
    input! {
        n: usize,
        k: usize,
        p: usize,
        an: [usize; n],
    }

    let first_half = an.iter().take(n / 2).copied().collect::<Vec<usize>>();
    let last_half = an.iter().skip(n / 2).copied().collect::<Vec<usize>>();
    // println!("{:?}", first_half);
    // println!("{:?}", last_half);

    // v_first[i]: i 個の商品を購入した結果の可変長配列
    let mut v_first = vec![vec![]; k + 1];
    for bit in 0..2u64.pow(first_half.len() as u32) {
        // println!("{}", bit);
        let mut cur_k = 0;
        let mut cur_p = 0;
        for i in 0..first_half.len() {
            if bit & (1 << i) > 0 {
                cur_k += 1;
                cur_p += first_half[i];
            }
        }
        if cur_k <= k && cur_p <= p {
            v_first[cur_k].push(cur_p);
        }
    }
    v_first.iter_mut().for_each(|v| v.sort());
    // println!("{:?}", v_first);

    let mut ans = 0;
    for bit in 0..2u64.pow(last_half.len() as u32) {
        let mut cur_k = 0;
        let mut cur_p = 0;
        for i in 0..last_half.len() {
            if bit & (1 << i) > 0 {
                cur_k += 1;
                cur_p += last_half[i];
            }
        }
        if cur_k <= k && cur_p <= p {
            let absent_k = k - cur_k;
            let absent_p = p - cur_p + 1;
            if !v_first[absent_k].is_empty() {
                // println!("  k: {}, p: {}", cur_k ,cur_p);
                ans += lower_bound(&v_first[absent_k], absent_p);
            }
        }
    }

    println!("{}", ans);
}
