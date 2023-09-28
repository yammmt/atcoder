// 12min 日本語

use proconio::input;

#[allow(clippy::needless_range_loop)]
fn main() {
    input! {
        n: usize,
        fn10: [[usize; 10]; n],
        pn11: [[i64; 11]; n],
    }

    let mut ans = std::i64::MIN / 2;
    for i in 1..2u32.pow(10) {
        let mut j = 0;
        let mut cur_i = i;
        let mut joisino_open = vec![false; 10];
        while cur_i > 0 {
            if cur_i % 2 == 1 {
                joisino_open[j] = true;
            }

            j += 1;
            cur_i /= 2;
        }

        let mut score = 0;
        // 店 j
        for j in 0..n {
            let mut covered = 0;
            for k in 0..10 {
                if joisino_open[k] && fn10[j][k] == 1 {
                    covered += 1;
                }
            }
            score += pn11[j][covered];
        }
        ans = ans.max(score);
    }

    println!("{}", ans);
}
