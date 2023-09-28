// 25.5min

use proconio::input;

fn main() {
    input! {
        k: usize,
        t: usize,
        mut at: [u64; t],
    }

    if t == 1 {
        println!("{}", k - 1);
        return;
    }

    at.sort_unstable();
    let sumwomax = at.iter().take(t - 1).sum::<u64>();
    if sumwomax >= at[t - 1] {
        println!("0");
    } else {
        println!("{}", at[t - 1] - sumwomax - 1);
    }
}
