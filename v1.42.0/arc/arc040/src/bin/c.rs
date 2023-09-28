// 30min (WA: 13.5min)
// WA: '.' が連続している必要はない

// 問題文が苦し
// 古いのでテストケースなし

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        mut sn: [Chars; n],
    }

    // 最大でも行につき 50 回を 100 行分のはず
    let mut ans = 0;
    for i in 0..n {
        let mut j = (n - 1) as isize;
        // println!("{:?}", sn);
        while j >= 0 {
            if sn[i][j as usize] == '.' {
                if i < n - 1 {
                    for k in j as usize..n {
                        sn[i + 1][k] = 'o';
                    }
                }
                while j >= 0 {
                    sn[i][j as usize] = 'o';
                    j -= 1;
                }
                ans += 1;
            }

            j -= 1;
        }
    }

    println!("{}", ans);
}
