// https://scrapbox.io/procon-kirokuyou/diverta2019-2_C_-_Successive_Subtraction_(500)
// :fu: :fu:
// 考察段階で場合分けさせる数問 最も苦手な組み合わせ

use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        mut an: [i64; n],
    }
    an.sort_unstable();

    // `vec` で正位置取ろうとすると符号が片方だけだった場合に幅がめんどくさくなる
    let mut posi = VecDeque::new();
    let mut nega = VecDeque::new();
    for a in &an {
        if *a < 0 {
            nega.push_back(*a);
        } else {
            posi.push_back(*a);
        }
    }

    if nega.is_empty() {
        nega.push_back(posi.pop_front().unwrap());
    } else if posi.is_empty() {
        posi.push_back(nega.pop_back().unwrap());
    }
    let mut nega_min = nega.pop_front().unwrap();
    let mut posi_max = posi.pop_back().unwrap();

    let mut ans_op = vec![];
    // 負方向に伸ばす
    while let Some(cur) = posi.pop_front() {
        ans_op.push((nega_min, cur));
        nega_min -= cur;
    }
    // 正方向に伸ばす
    while let Some(cur) = nega.pop_front() {
        ans_op.push((posi_max, cur));
        posi_max -= cur;
    }

    println!("{}", posi_max - nega_min);
    ans_op.push((posi_max, nega_min));
    for s in &ans_op {
        println!("{} {}", s.0, s.1);
    }
}
