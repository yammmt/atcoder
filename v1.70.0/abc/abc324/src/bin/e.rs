use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut t: Chars,
        mut sn: [Chars; n],
    }

    // 大分苦手
    // bac-*
    // ba-c
    // b-ac
    // *-bac

    let cnt = |tt: &Vec<char>, ss: &Vec<Vec<char>>| {
        let t_len = tt.len();
        let mut cnt_char = Vec::with_capacity(ss.len());
        for s in ss {
            let mut t_i = 0;
            for c in s {
                if *c == tt[t_i] {
                    t_i += 1;
                    if t_i == t_len {
                        break;
                    }
                }
            }
            cnt_char.push(t_i);
        }
        cnt_char
    };

    // 先頭から n 文字もっている文字列の数
    let cnt_front = cnt(&t, &sn);

    // 後ろから n 文字もっている文字列の数
    t.reverse();
    let mut rsn = Vec::with_capacity(n);
    for s in sn {
        let mut r = s.clone();
        r.reverse();
        rsn.push(r);
    }
    let cnt_back = cnt(&t, &rsn);

    let mut element_back = vec![0; t.len() + 1];
    for c in cnt_back {
        element_back[c] += 1;
    }

    let mut cusum_back = vec![0; t.len() + 1];
    cusum_back[t.len()] = element_back[t.len()];
    for i in (0..t.len()).rev() {
        cusum_back[i] = cusum_back[i + 1] + element_back[i];
    }

    // 自身を二度連結する場合は許される
    let mut ans = 0u64;
    for i in 0..n {
        let left = t.len() - cnt_front[i];
        ans += cusum_back[left];
    }

    println!("{ans}");
}
