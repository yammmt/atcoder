use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    // 自分より後ろに自分と異なる文字が何種類あるか
    let mut cnt = vec![vec![0; 26]; s.len()];
    for (i, c) in s.iter().enumerate() {
        if i != 0 {
            for j in 0..26 {
                cnt[i][j] = cnt[i - 1][j];
            }
        }

        cnt[i][(*c as u8 - b'a') as usize] += 1;
    }

    let mut ans = 0u64;
    for (i, c) in s.iter().enumerate() {
        let cu = (*c as u8 - b'a') as usize;

        for j in 0..26 {
            if j == cu {
                continue;
            }

            ans += cnt[s.len() - 1][j] - cnt[i][j];
        }
    }

    // 交換しても変わらない場合
    for i in 0..26 {
        if cnt[s.len() - 1][i] > 1 {
            ans += 1;
            break;
        }
    }

    println!("{ans}");
}
