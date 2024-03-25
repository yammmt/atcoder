use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        w: usize,
        b: usize,
    }

    // 元の文字列長 12 のどれかを先頭として 200 字数える
    // 境界を考えるのがだるいのでとりあえず +1
    let piano_len_max = 12 + 200 + 1;
    let piano = "wbwbwwbwbwbw".chars().collect::<Vec<char>>();
    let mut vp = vec![];
    for i in 0..piano_len_max {
        vp.push(piano[i % piano.len()]);
    }

    let mut cnt_w = 0;
    let mut cnt_b = 0;
    let mut vdq = VecDeque::new();
    for i in 0..piano_len_max {
        match vp[i] {
            'w' => {
                vdq.push_back('w');
                cnt_w += 1;
            }
            'b' => {
                vdq.push_back('b');
                cnt_b += 1;
            }
            _ => unreachable!(),
        }
        if cnt_w + cnt_b < w + b {
            continue;
        }

        if cnt_w == w && cnt_b == b {
            println!("Yes");
            return;
        }

        match vdq.pop_front() {
            Some('w') => cnt_w -= 1,
            Some('b') => cnt_b -= 1,
            _ => unreachable!(),
        }
    }

    println!("No");
}
