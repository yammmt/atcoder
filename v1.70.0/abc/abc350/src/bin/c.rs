// 3
// 2 3 1

use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut an: [Usize1; n],
    }

    let mut pos = vec![0; n];
    for (i, a) in an.iter().enumerate() {
        pos[*a] = i;
    }

    let mut ans = vec![];
    for i in 0..n {
        if an[i] == i {
            continue;
        }

        let j = pos[i];
        ans.push((i + 1, j + 1));

        // i <=> j の swap だから
        pos[an[i]] = j;
        pos[an[j]] = i;

        an.swap(i, j);
    }

    println!("{}", ans.len());
    for a in ans {
        println!("{} {}", a.0, a.1);
    }
}
