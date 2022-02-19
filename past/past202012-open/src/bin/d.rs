// :fu: 22-02 めちゃくちゃいやな問題だが PAST らしさはある

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        mut sn: [Chars; n],
    }

    sn.sort_unstable_by(|a, b| {
        let mut a_0_len = 0;
        for aa in a {
            if *aa == '0' {
                a_0_len += 1;
            } else {
                break;
            }
        }

        let mut b_0_len = 0;
        for bb in b {
            if *bb == '0' {
                b_0_len += 1;
            } else {
                break;
            }
        }

        let a_len = a.len() - a_0_len;
        let b_len = b.len() - b_0_len;
        if a_len != b_len {
            // 0 埋め考慮以前に桁数で判定できる
            a_len.cmp(&b_len)
        } else {
            let a_actual = a.iter().skip(a_0_len).collect::<Vec<&char>>();
            let b_actual = b.iter().skip(b_0_len).collect::<Vec<&char>>();
            if a_actual != b_actual {
                a_actual.cmp(&b_actual)
            } else {
                b_0_len.cmp(&a_0_len)
            }
        }
    });

    for s in &sn {
        println!("{}", s.iter().collect::<String>());
    }
}
