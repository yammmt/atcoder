// :fu: 21-10 数問 + 日本語問題

use proconio::input;

fn main() {
    input! {
        n: usize,
        abn: [(f64, f64); n],
    }

    let mut goal_time = 0.0;
    for ab in &abn {
        goal_time += ab.0 / ab.1;
    }
    goal_time /= 2.0;

    let mut cur_dist = 0.0;
    let mut cur_time = 0.0;
    for ab in &abn {
        if cur_time + ab.0 / ab.1 >= goal_time {
            println!("{}", cur_dist + (goal_time - cur_time) * ab.1);
            return;
        } else {
            cur_dist += ab.0;
            cur_time += ab.0 / ab.1;
        }
    }
}
