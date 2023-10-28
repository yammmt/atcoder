use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: i64,
        x: i64,
        an: [i64; n],
    }
    let a_sum = an.iter().sum::<i64>();

    let mut discount = 0;
    let mut k_used = 0;
    let mut k_fraction = Vec::with_capacity(n);
    // 負にならないだけ全部引く
    for a in an {
        let can_use = (a / x).min(k - k_used);
        k_used += can_use;
        discount += can_use * x;
        k_fraction.push(a - can_use * x);
    }
    k_fraction.sort_unstable();
    k_fraction.reverse();

    discount += k_fraction.iter().take((k - k_used) as usize).sum::<i64>();

    println!("{}", a_sum - discount);
}
