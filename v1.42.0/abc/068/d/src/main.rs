// :fu: 21-05 構築

use proconio::input;

fn main() {
    input! {
        k: usize,
    }

    let q = k / 50;
    let r = k % 50;

    // 全要素に操作を適用すると値が - 1 されるので
    // 要素数 N (50) に対して端数がなければ 49 + k / 50 を出しておけば良い
    let mut ans = vec![49 + q; 50];

    // 端数分余計に操作する
    ans.iter_mut().take(r).for_each(|a| *a += 1);

    // 端数操作で超えてしまう分を引く
    ans.iter_mut().skip(r).for_each(|a| *a -= r);

    println!("{}", ans.len());
    for (i, a) in ans.iter().enumerate() {
        print!("{}", a);
        if i == ans.len() - 1 {
            println!();
        } else {
            print!(" ");
        }
    }
}
