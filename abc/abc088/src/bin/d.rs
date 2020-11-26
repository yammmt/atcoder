// 14.5min

use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut counts = vec![vec![std::usize::MAX; w]; h];
    counts[0][0] = 1;
    let mut vdq = VecDeque::new();
    vdq.push_back((0, 0));
    while !vdq.is_empty() {
        let cur = vdq.pop_front().unwrap();
        if cur.0 > 0 && s[cur.0 - 1][cur.1] == '.' && counts[cur.0 - 1][cur.1] == std::usize::MAX {
            vdq.push_back((cur.0 - 1, cur.1));
            counts[cur.0 - 1][cur.1] = counts[cur.0][cur.1] + 1;
        }
        if cur.0 < h - 1 && s[cur.0 + 1][cur.1] == '.' && counts[cur.0 + 1][cur.1] == std::usize::MAX {
            vdq.push_back((cur.0 + 1, cur.1));
            counts[cur.0 + 1][cur.1] = counts[cur.0][cur.1] + 1;
        }
        if cur.1 > 0 && s[cur.0][cur.1 - 1] == '.' && counts[cur.0][cur.1 - 1] == std::usize::MAX {
            vdq.push_back((cur.0, cur.1 - 1));
            counts[cur.0][cur.1 - 1] = counts[cur.0][cur.1] + 1;
        }
        if cur.1 < w - 1 && s[cur.0][cur.1 + 1] == '.' && counts[cur.0][cur.1 + 1] == std::usize::MAX {
            vdq.push_back((cur.0, cur.1 + 1));
            counts[cur.0][cur.1 + 1] = counts[cur.0][cur.1] + 1;
        }
    }

    if counts[h - 1][w - 1] == std::usize::MAX {
        println!("-1");
        return;
    }

    let default_black_num = s.iter().fold(0, |su, row| {
        su + row.iter().filter(|&&c| c == '#').count()
    });
    println!(
        "{}",
        h * w - default_black_num - counts[h - 1][w - 1]
    );
}
