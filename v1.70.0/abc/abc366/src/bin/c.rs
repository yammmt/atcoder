use proconio::fastout;
use proconio::input;

const X_MAX: usize = 1_000_001;

#[fastout]
fn main() {
    input! {
        q: usize,
    }

    // 集合型を使ってもよいが, x の範囲が狭いので配列で見た方が高速

    let mut ans = 0;
    let mut ball_cnt = [0; X_MAX];
    for _ in 0..q {
        input! {
            n: usize,
        }
        match n {
            1 => {
                input! {
                    x: usize,
                }
                ball_cnt[x] += 1;
                if ball_cnt[x] == 1 {
                    ans += 1;
                }
            }
            2 => {
                input! {
                    x: usize,
                }
                ball_cnt[x] -= 1;
                if ball_cnt[x] == 0 {
                    ans -= 1;
                }
            }
            3 => println!("{ans}"),
            _ => unreachable!(),
        }
    }
}
