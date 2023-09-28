// 12min 1WA (8min)
// WA: フェールセーフで n < 2 入れたら return 書き忘れた

use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [i64; n],
    }

    let mut ans = 1;
    let mut up_appeared = false;
    let mut down_appeared = false;
    for i in 1..n {
        if an[i] > an[i - 1] {
            up_appeared = true;
        } else if an[i] < an[i - 1] {
            down_appeared = true;
        }
        if up_appeared && down_appeared {
            ans += 1;
            up_appeared = false;
            down_appeared = false;
        }
    }

    println!("{}", ans);
}
