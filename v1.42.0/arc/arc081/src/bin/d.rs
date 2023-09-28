use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s1: Chars,
        _s2: Chars,
    }
    let d = 10u64.pow(9) + 7;

    let mut is_prev_side;
    let mut nidx;
    let mut ans;
    if n == 1 || s1[0] != s1[1] {
        ans = 3;
        is_prev_side = false;
        nidx = 1;
    } else {
        ans = 6;
        is_prev_side = true;
        nidx = 2;
    };

    while nidx < n {
        if is_prev_side {
            if nidx + 1 >= n || s1[nidx] != s1[nidx + 1] {
                is_prev_side = false;
                nidx += 1;
            } else {
                ans = (ans * 3) % d;
                is_prev_side = true;
                nidx += 2;
            }
        } else {
            ans = (ans * 2) % d;
            if nidx + 1 >= n || s1[nidx] != s1[nidx + 1] {
                is_prev_side = false;
                nidx += 1;
            } else {
                is_prev_side = true;
                nidx += 2;
            }
        }
    }

    println!("{}", ans);
}
