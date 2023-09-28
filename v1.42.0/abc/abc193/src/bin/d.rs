// :fu: 21-02 数問ではあるが焦らない

use proconio::input;
use proconio::marker::Chars;

fn point(cards: &[i64; 9]) -> i64 {
    let mut ret = 0;
    for i in 0..9 {
        ret += (i as i64 + 1) * (10u64.pow(cards[i] as u32) as i64);
    }
    ret
}

fn main() {
    input! {
        k: usize,
        s: Chars,
        t: Chars,
    }

    let mut left = [k; 9];

    let mut tkhs = [0; 9];
    for i in 0..4 {
        let c = (s[i] as u8 - b'0') as i64;
        tkhs[c as usize - 1] += 1;
        left[c as usize - 1] -= 1;
    }

    let mut aoki = [0; 9];
    for i in 0..4 {
        let c = (t[i] as u8 - b'0') as i64;
        aoki[c as usize - 1] += 1;
        left[c as usize - 1] -= 1;
    }

    let mut bunshi = 0.0;
    let mut bunbo = 0.0;
    // tkhs の残り一枚が i + 1 とする
    for i in 0..9 {
        if left[i] == 0 {
            // 存在しない組み合わせ
            continue;
        }

        let mut left_new = left.clone();

        let mut tkhs_new = tkhs.clone();
        tkhs_new[i] += 1;
        left_new[i] -= 1;
        let tkhs_pts = point(&tkhs_new);

        for j in 0..9 {
            if left_new[j] == 0 {
                // 存在しない組み合わせ
                continue;
            }

            let mut aoki_new = aoki.clone();
            aoki_new[j] += 1;
            let aoki_pts = point(&aoki_new);
            if tkhs_pts > aoki_pts {
                bunshi += (left[i as usize] as f64) * (left_new[j as usize] as f64);
            }
            bunbo += (left[i as usize] as f64) * (left_new[j as usize] as f64);
        }
    }

    println!("{}", bunshi / bunbo);
}
