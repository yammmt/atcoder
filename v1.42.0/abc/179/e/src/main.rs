// -*- coding:utf-8-unix -*-

// WA: an のサイズを n まんまにしてメモリ確保できず RE

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut x: u64,
        m: u64,
    }

    let mut an = vec![0; m as usize];
    let mut appeared = vec![std::usize::MAX; m as usize];
    let mut ans = 0;
    for i in 0..n {
        if appeared[x as usize] != std::usize::MAX {
            // cycle
            let loop_left = (n - i) as u64;
            let cycle_len = (i - appeared[x as usize]) as u64;
            let cycle_left = loop_left / cycle_len;
            let cycle_rest = loop_left % cycle_len;
            let cycle_sum = an
                .iter()
                .skip(appeared[x as usize])
                .take(cycle_len as usize)
                .sum::<u64>();
            ans += cycle_sum * cycle_left;
            for j in 0..cycle_rest {
                ans += an[appeared[x as usize] + j as usize];
            }
            break;
        }

        an[i] = x;
        appeared[x as usize] = i;
        ans += an[i];
        x = an[i] * an[i] % m;
    }

    println!("{}", ans);
}
