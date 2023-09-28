// 30min editorial とは違うが変な解法ではなさそう

use proconio::input;

fn main() {
    input! {
        n: usize,
        c: usize,
        an: [usize; n],
    }

    let mut ans = vec![0usize; c];
    let mut last_pos = vec![0; c + 1];
    for (i, a) in an.iter().enumerate() {
        // println!("{} x {}", i - last_pos[*a] + 1, n - i);
        ans[*a - 1] += (i - last_pos[*a] + 1) * (n - i);
        last_pos[*a] = i + 1;
    }

    for a in &ans {
        println!("{}", a);
    }
}
