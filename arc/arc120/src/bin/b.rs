// A の方が難しい？ 同じ 400 点ではあるが

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        shw: [Chars; h],
    }
    let d = 998_244_353;

    let mut ans = 1u64;
    // 斜め きれいに書けない
    // 左上から右へ
    for i in 0..w {
        let mut r_num = 0;
        let mut b_num = 0;

        let mut cur_h = 0;
        let mut cur_w = i;
        loop {
            match shw[cur_h][cur_w] {
                'R' => r_num += 1,
                'B' => b_num += 1,
                _ => {},
            }

            if cur_h == h - 1 || cur_w == 0 {
                break;
            }
            cur_h += 1;
            cur_w -= 1;
        }

        if r_num > 0 && b_num > 0 {
            ans = 0;
        } else if r_num == 0 && b_num == 0 {
            // すべてを R か B に塗る
            ans = (ans * 2) % d;
        }
    }

    // 右上から下へ
    for i in 1..h {
        let mut r_num = 0;
        let mut b_num = 0;

        let mut cur_h = i;
        let mut cur_w = w - 1;
        loop {
            match shw[cur_h][cur_w] {
                'R' => r_num += 1,
                'B' => b_num += 1,
                _ => {},
            }

            if cur_h == h - 1 || cur_w == 0 {
                break;
            }
            cur_h += 1;
            cur_w -= 1;
        }

        if r_num > 0 && b_num > 0 {
            ans = 0;
        } else if r_num == 0 && b_num == 0 {
            // すべてを R か B に塗る
            ans = (ans * 2) % d;
        }
    }

    println!("{}", ans);
}
