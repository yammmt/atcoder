use proconio::input;
use std::cmp::Ordering;

fn main() {
    input! {
        h: usize,
        w: usize,
        mut ahw: [[isize; w]; h],
        bhw: [[isize; w]; h],
    }

    // 操作回数は高が知れている はず
    let mut ans = 0;
    for i in 0..h - 1 {
        for j in 0..w - 1 {
            let diff = (ahw[i][j] - bhw[i][j]).abs();
            ans += diff;

            match ahw[i][j].cmp(&bhw[i][j]) {
                Ordering::Less => {
                    ahw[i][j] += diff;
                    ahw[i + 1][j] += diff;
                    ahw[i][j + 1] += diff;
                    ahw[i + 1][j + 1] += diff;
                }
                Ordering::Greater => {
                    ahw[i + 1][j + 1] -= diff;
                    ahw[i][j] -= diff;
                    ahw[i + 1][j] -= diff;
                    ahw[i][j + 1] -= diff;
                }
                Ordering::Equal => {}
            }
        }
    }

    for i in 0..h {
        if ahw[i][w - 1] != bhw[i][w - 1] {
            println!("No");
            return;
        }
    }

    for i in 0..w {
        if ahw[h - 1][i] != bhw[h - 1][i] {
            println!("No");
            return;
        }
    }

    println!("Yes");
    println!("{}", ans);
}
