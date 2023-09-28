// "ab2c1 13" => b

use proconio::input;
use proconio::marker::Chars;

// 32bit 環境ではもう少し複雑に書かねばエラー
const XMAX: usize = 1_000_000_000_000_001;

fn main() {
    input! {
        s: Chars,
        x: usize,
    }

    // その字を実行し終わった時点で何文字出力したか
    let mut out_cnt = Vec::with_capacity(s.len());
    let mut cur_cnt = 0;
    for c in &s {
        if c.is_numeric() {
            let xx = (*c as u8 - b'0') as usize;
            cur_cnt = (cur_cnt * (xx + 1)).min(XMAX);
            out_cnt.push(cur_cnt);
        } else {
            cur_cnt = (cur_cnt + 1).min(XMAX);
            out_cnt.push(cur_cnt);
        }
    }
    // println!("{:?}", out_cnt);

    // cur_i 番目の命令で cur_x 番目に出力された文字が答え
    let mut cur_i = 0;
    let mut cur_x = x;
    for (i, &n) in out_cnt.iter().enumerate() {
        if n >= x {
            cur_i = i;
            break;
        }
    }

    loop {
        // println!("cur_i: {}", cur_i);
        // println!("cur_x: {}", cur_x);

        if s[cur_i].is_numeric() {
            cur_i -= 1;
            cur_x = (cur_x - 1) % out_cnt[cur_i] + 1;
        } else if cur_x == out_cnt[cur_i] {
            println!("{}", s[cur_i]);
            return;
        } else {
            cur_i -= 1;
        }
    }
}
