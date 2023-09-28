// :fu: 21-05 知ってしまうとやり直したところでかもしれない
// 貪欲の方が発想実装共に楽そうだが

use proconio::input;

fn main() {
    input! {
        k: usize,
        n: i64,
        m: i64,
        ak: [i64; k],
    }

    let mut ans = vec![];
    let mut sumb = 0;
    for (i, a) in ak.iter().enumerate() {
        let b = m * *a / n;
        ans.push(((m * *a % n) as f64 / n as f64, b, i));
        sumb += b;
    }
    // println!("{:?}", ans);
    ans.sort_by(|a, b| {
        a.0.partial_cmp(&b.0).unwrap()
    });
    ans.reverse();
    for i in 0..m - sumb {
        ans[i as usize].1 += 1;
    }
    ans.sort_by(|a, b| {
        a.2.cmp(&b.2)
    });

    for (i, a) in ans.iter().enumerate() {
        print!("{}", a.1);
        if i == ans.len() - 1 {
            println!();
        } else {
            print!(" ");
        }
    }
}
