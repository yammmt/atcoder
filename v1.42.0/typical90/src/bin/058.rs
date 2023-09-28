// 解けるけれど実装にちょっと手間取る

use proconio::input;

const DUMMY: usize = std::usize::MAX / 3;

fn calc(mut n: usize) -> usize {
    let mut y = n;
    while n > 0 {
        y += n % 10;
        n /= 10;
    }
    y % 100_000
}

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut appeared = vec![DUMMY; 100_001];
    let mut n_th = vec![DUMMY; 100_001];
    appeared[0] = n;
    n_th[n] = 0;

    let mut cur_n = n;
    let mut i = 1;
    loop {
        let next_n = calc(cur_n);
        // println!("{}", next_n);
        if i == k {
            println!("{}", next_n);
            return;
        } else if n_th[next_n] != DUMMY {
            let loop_len = i - n_th[next_n];
            let offset = n_th[next_n];
            let fraction = (k - i) % loop_len;
            println!("{}", appeared[offset + fraction]);
            return;
        } else {
            appeared[i] = next_n;
            n_th[next_n] = i;
        }
        cur_n = next_n;
        i += 1;
    }
}
