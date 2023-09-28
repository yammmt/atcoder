// ARC に出そう

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    // 点 1 の下に全点入れた場合が上限値
    let mut cur_k = (n - 1) * (n - 2) / 2;
    if k > cur_k {
        println!("-1");
        return;
    }

    let mut ans = (2..n + 1).map(|i| format!("1 {}", i)).collect::<Vec<String>>();
    let mut i = 2;
    let mut j = 3;
    while cur_k != k {
        ans.push(format!("{} {}", i, j));
        if j == n {
            i += 1;
            j = i + 1;
        } else {
            j += 1;
        }
        cur_k -= 1;
    }

    println!("{}", ans.len());
    for a in &ans {
        println!("{}", a);
    }
}
