// :fu: :fu: 21-05 nCr 高速化が要る緑中盤は嘘

use proconio::input;

fn main() {
    input! {
        mut a: usize,
        mut b: usize,
        mut k: usize,
    }
    k -= 1;

    let mut ncr = vec![vec![0; a + b + 2]; a + b + 2];
    ncr[0][0] = 1;
    for i in 0..a + b + 1 {
        for j in 0..i + 1 {
            ncr[i + 1][j] += ncr[i][j];
            ncr[i + 1][j + 1] += ncr[i][j];
        }
    }

    let mut ans = vec![];
    for _ in 0..a + b {
        if a > 0 {
            let acb = ncr[a + b - 1][b];
            // println!("{}C{} = {}", a + b - 1, b, acb);
            if acb > k {
                ans.push('a');
                a -= 1;
            } else {
                ans.push('b');
                b -= 1;
                k -= acb;
            }
        } else {
            ans.push('b');
            b -= 1;
        }
    }

    println!("{}", ans.iter().collect::<String>());
}
