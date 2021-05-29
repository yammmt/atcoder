// 統計上は灰だが, 参加しやすい時間帯にも関わらず提出者が前の ARC より大分減っており...
// Simple Math 3 ほどでないが

use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [i64; n],
    }

    let mut ans = 0;
    let mut a_max = 0;
    let mut a_sum = 0;
    for (i, a) in an.iter().enumerate() {
        a_max = a_max.max(*a);
        ans += a_sum;
        ans += *a;
        println!("{}", ans + a_max * (i as i64 + 1));
        a_sum += *a;
    }
}
