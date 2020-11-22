// 36min 3WA (1TLE)
// TLE: 約数列挙が O(N sqrt(N)), N <= 2 x 10^5 なら間に合うと思ったら間に合わなかった
// WA: 約数の数え上げを素因数分解の範囲で判定してしまっていた

use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [u64; n],
    }

    let mut affected = vec![vec![]; n];
    let mut p = 1;
    while p * 2 <= n {
        let mut x = 2;
        while p * x <= n {
            affected[p * x - 1].push(p - 1);
            x += 1;
        }
        p += 1;
    }
    // println!("affected: {:?}", affected);

    let mut ball = vec![0; n];
    let mut ans = vec![];
    for i in (0..n).rev() {
        if ball[i] % 2 == an[i] {
            continue;
        }

        ans.push(i + 1);
        ball[i] += 1;
        for j in &affected[i] {
            ball[*j] += 1;
        }
    }
    // println!("ans: {:?}", ans);

    println!("{}", ans.len());
    if !ans.is_empty() {
        // 順不同
        // ans.reverse();
        for i in 0..ans.len() {
            print!("{}", ans[i]);
            if i != ans.len() - 1 {
                print!(" ");
            }
        }
        println!();
    }
}
