// 50min

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut an: [i64; n],
    }
    an.sort_unstable();

    let mut ans = 0;
    match n % 2 {
        0 => {
            for (i, a) in an.iter().enumerate() {
                if i < n / 2 - 1 {
                    ans -= 2 * *a;
                } else if i == n / 2 - 1 {
                    ans -= *a;
                } else if i == n / 2 {
                    ans += *a;
                } else {
                    ans += 2 * *a;
                }
            }
        }
        1 => {
            let mut front = 0;
            let mut rear = 0;
            for (i, a) in an.iter().enumerate() {
                if i < n / 2 - 1 {
                    front -= 2 * *a;
                    rear -= 2 * *a;
                } else if i == n / 2 - 1 {
                    front -= *a;
                    rear -= 2 * *a;
                } else if i == n / 2 {
                    front -= *a;
                    rear += *a;
                } else if i == n / 2 + 1 {
                    front += 2 * *a;
                    rear += *a;
                } else {
                    front += 2 * *a;
                    rear += 2 * *a;
                }
            }
            ans = front.max(rear);
        }
        _ => unreachable!(),
    }

    println!("{}", ans);
}
