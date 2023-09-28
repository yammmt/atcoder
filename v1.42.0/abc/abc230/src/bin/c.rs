// 日本語

use proconio::input;

fn main() {
    input! {
        _n: usize,
        a: isize,
        b: isize,
        p: usize,
        q: usize,
        r: usize,
        s: usize,
    }

    // a = 1, b = 1, n = 10^18 で愚直に見ると TLE

    // 対象のマスが (A, B) よりどれだけ離れているかを見る
    // A/B からの差分が同じでなければ白のまま
    // すべての "整数" であり負たり得る
    let mut ans = vec![vec!['.'; s - r + 1]; q - p + 1];
    for i in 0..q - p + 1 {
        let cur_i = (p + i) as isize;
        let k_i = cur_i - a;
        for j in 0..s - r + 1 {
            let cur_j = (r + j) as isize;
            // println!("{} {}", cur_i, cur_j);
            let k_j = cur_j - b;
            if k_i == k_j {
                // cond 1
                ans[i][j] = '#';
            } else if k_i == -k_j {
                // cond 2
                ans[i][j] = '#';
            }
        }
    }

    for a in &ans {
        println!("{}", a.iter().collect::<String>());
    }
}
