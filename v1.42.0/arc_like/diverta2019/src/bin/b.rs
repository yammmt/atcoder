// 9min
// なぜか組み合わせ計算しようとしていた

use proconio::input;

fn main() {
    input! {
        r: u64,
        g: u64,
        b: u64,
        n: u64,
    }

    let mut ans = 0;
    let rmax = n / r + 1;
    let gmax = n / g + 1;
    for i in 0..rmax {
        for j in 0..gmax {
            let cur = i * r + j * g;
            if cur > n {
                break;
            }

            if (n - cur) % b == 0 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
