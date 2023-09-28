// しゃくとり法 苦手

use proconio::input;

fn calc_score(x: i64, total: i64) -> i64 {
    (x - (total - x)).abs()
}

fn main() {
    input! {
        n: usize,
        an: [i64; n],
    }

    // 愚直は O(N^2)
    // 食べる個数が決まっているならば O(N) でやるだけ
    // 美味しさの総和は単調増加だから尺取法が使える はず
    let mut ans = std::i64::MAX / 2;
    let mut right = 0;
    let mut cur_x = 0;
    let point_sum = an.iter().sum::<i64>();
    for left in 0..n {
        // 良くなるだけ進める
        while calc_score(cur_x + an[right], point_sum) <= calc_score(cur_x, point_sum) {
            cur_x += an[right];
            right = (right + 1) % n;
            ans = ans.min(calc_score(cur_x, point_sum));
        }

        cur_x -= an[left];
    }

    println!("{}", ans);
}
