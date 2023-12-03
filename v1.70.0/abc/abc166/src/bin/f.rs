// 愚直に書こうとすると超重実装になる, Rust の文字列の仕様も辛い
// 簡潔: https://atcoder.jp/contests/abc166/submissions/34713817

use proconio::fastout;
use proconio::input;
use proconio::marker::Bytes;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut abc: [usize; 3],
        sn: [Bytes; n],
    }

    let snn = sn
        .iter()
        .map(|cc| ((cc[0] - b'A') as usize, (cc[1] - b'A') as usize))
        .collect::<Vec<_>>();
    let mut ans = vec![];
    let mut add_ans = |i, j, v: &mut Vec<_>| {
        ans.push(i);
        v[i] += 1;
        v[j] -= 1;
    };

    for i in 0..n {
        let (p, q) = snn[i];
        let cnt_f = abc[p];
        let cnt_s = abc[q];

        if cnt_f == 0 && cnt_s == 0 {
            println!("No");
            return;
        } else if cnt_f == 0 {
            add_ans(p, q, &mut abc);
        } else if cnt_s == 0 {
            add_ans(q, p, &mut abc);
        } else if i + 1 < n {
            let (p_nxt, q_nxt) = snn[i + 1];
            if p == p_nxt || p == q_nxt {
                add_ans(p, q, &mut abc);
            } else {
                add_ans(q, p, &mut abc);
            }
        } else {
            add_ans(p, q, &mut abc);
        }
    }

    println!("Yes");
    for a in ans {
        println!("{}", (b'A' + a as u8) as char);
    }
}
