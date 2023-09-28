// 6.5min 1WA
// WA: `an[i] -= same_pairs` としていた

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut an: [i64; n],
    }

    let mut ans = 0;
    for i in 0..n {
        let same_pairs = an[i] / 2;
        ans += same_pairs;
        an[i] -= 2 * same_pairs;

        if an[i] > 0 && i < n - 1 {
            let next_pairs = an[i].min(an[i + 1] / 2);
            ans += next_pairs;
            an[i] -= next_pairs;
            an[i + 1] -= next_pairs;
        }
    }
    println!("{}", ans);
}
