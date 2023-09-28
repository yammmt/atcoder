// :fu: 21-06 投票でも茶色相当だが個人的にはもっと難しい

use proconio::input;

fn main() {
    input! {
        n: usize,
        an6: [[u64; 6]; n],
    }
    let d = 1_000_000_007;

    let mut ans = 1;
    for an in &an6 {
        ans = (ans * an.iter().sum::<u64>()) % d;
    }

    println!("{}", ans);
}
