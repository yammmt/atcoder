// 17min
// 1WA: k == 1

use proconio::input;

fn find_streak(an: &Vec<i64>, b: usize, k: usize) -> usize {
    let mut streak = 1;
    for i in b + 1..an.len() {
        if an[i] > an[i - 1] {
            streak += 1;
        } else {
            streak = 1;
        }
        if streak == k {
            return i;
        }
    }
    // None
    an.len() + 1
}

fn main() {
    input! {
        n: usize,
        k: usize,
        an: [i64; n],
    }

    if k == 1 {
        println!("{}", n);
        return;
    }

    let mut rear = find_streak(&an, 0, k);
    let mut ans = 0;
    while rear < n {
        ans += 1;
        if rear >= n - 1 {
            break;
        }

        if an[rear + 1] > an[rear] {
            rear += 1;
        } else {
            rear = find_streak(&an, rear, k);
        }
    }

    println!("{}", ans);
}
