// :fu:
// https://webbibouroku.com/Blog/Article/atcoder-abc147

use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [u64; n],
    }
    let d = 10u64.pow(9) + 7;

    let mut one_num = vec![0; 61];
    let mut zero_num = vec![0; 61];
    for a in &an {
        let mut aa = *a;
        for i in 0..61 {
            match aa & 1 {
                1 => one_num[i] += 1,
                _ => zero_num[i] += 1,
            };
            aa >>= 1;
        }
    }

    let mut two_n = Vec::with_capacity(61);
    two_n.push(1);
    for i in 1..61 {
        two_n.push((2 * two_n[i - 1]) % d);
    }

    let mut ans = 0;
    for i in 0..61 {
        ans = (ans + (((two_n[i] * one_num[i]) % d) * zero_num[i]) % d) % d;
    }

    println!("{}", ans);
}
