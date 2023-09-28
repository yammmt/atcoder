// :fu: :fu: :fu: 21-03 数問

use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = vec![1; n];
    for i in 1..n {
        let mut n = i + 1;
        let mut p = 2;
        while p * p <= n {
            while n % p == 0 {
                ans[i] += 1;
                n /= p;
            }
            p += 1;
        }
        if n > 1 {
            ans[i] += 1;
        }
    }

    for (i, a) in ans.iter().enumerate() {
        print!("{}", a);
        if i == n - 1 {
            println!();
        } else {
            print!(" ");
        }
    }
}
