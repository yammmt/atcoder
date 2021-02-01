// 33min
// 場合分けというかインデックス
// 想定解は二分探索ではない

use proconio::input;

// v[i] >= k を満たす最小の i を返す (or k 以下である v の要素数)
// 見つからなければ `v.len()` を返す
// `v` はソートされていること
fn lower_bound(v: &[u32], k: u32) -> usize {
    let mut low: isize = -1;
    let mut high = v.len() as isize;

    while high - low > 1 {
        let mid = (low + high) / 2;
        if v[mid as usize] >= k {
            high = mid;
        } else {
            low = mid;
        }
    }
    high as usize
}

fn main() {
    input! {
        n: usize,
        rhn: [(u32, u32); n],
    }

    let mut rhn_cloned = rhn.clone();
    rhn_cloned.sort();
    // println!("{:?}", rhn_cloned);
    let mut rate = vec![];
    let mut hand = vec![];
    for rh in &rhn_cloned {
        rate.push(rh.0);
        hand.push(rh.1);
    }

    for rh in &rhn {
        // println!("{:?}", rh);
        let win_rate_idx = lower_bound(&rate, rh.0);
        let lose_rate_idx = lower_bound(&rate, rh.0 + 1);
        // println!("  {} - {} - x", win_rate_idx, n - lose_rate_idx);

        let draw_from = win_rate_idx;
        let draw_to = lose_rate_idx;
        // println!("  draw: [{}, {})", draw_from, draw_to);
        let draw_g = lower_bound(&hand[draw_from..draw_to], 2);
        let draw_c = lower_bound(&hand[draw_from..draw_to], 3) - draw_g;
        let draw_p = draw_to - (draw_from + draw_g + draw_c);
        // println!("  {} {} {}", draw_g, draw_c, draw_p);

        let (draw_win, draw_lose) = match rh.1 {
            // g
            1 => (draw_c, draw_p),
            // c
            2 => (draw_p, draw_g),
            // p
            3 => (draw_g, draw_c),
            _ => unreachable!(),
        };

        let win = win_rate_idx + draw_win;
        let lose = (n - lose_rate_idx) + draw_lose;
        println!(
            "{} {} {}",
            win,
            lose,
            n - (win + lose) - 1 // -1 for self
        );
    }
}
