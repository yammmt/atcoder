use proconio::fastout;
use proconio::input;
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        n: usize,
        d: usize,
        a: isize,
        mut xhn: [(usize, isize); n],
    }
    xhn.sort_unstable();

    let mut ans = 0;
    let mut dmg_cur = 0isize;
    // (ダメージの変わる x, ダメージ差分)
    let mut dmg_changes: VecDeque<(usize, isize)> = VecDeque::new();
    for xh in xhn {
        let x = xh.0;
        let h = xh.1;
        while let Some(p) = dmg_changes.pop_front() {
            if p.0 > x {
                dmg_changes.push_front(p);
                break;
            }

            dmg_cur += p.1;
        }

        if dmg_cur >= h {
            // すでに死んでいるので何もしない
            continue;
        } else {
            let h_left = h - dmg_cur;
            let attack_num = (h_left + a - 1) / a;
            let dmg_diff = attack_num * a;
            ans += attack_num;
            dmg_cur += dmg_diff;
            dmg_changes.push_back((x + 2 * d + 1, -dmg_diff));
        }
    }

    println!("{ans}");
}
