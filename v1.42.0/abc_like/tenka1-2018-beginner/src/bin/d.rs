// 21min 2WA
// WA: "Yes" を出し忘れた + n == 1

use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    if n == 1 {
        println!("Yes");
        println!("2");
        println!("1 1");
        println!("1 1");
        return;
    }

    let mut k = 0;
    for i in 1..n + 1 {
        if i * (i - 1) == 2 * n {
            k = i;
            break;
        }
    }

    if k == 0 {
        println!("No");
        return;
    }

    let mut ans = vec![vec![]; k];
    let mut i = 0;
    let mut j = 1;
    // 集合 i/j の共通要素
    for nn in 1..n + 1 {
        // println!("{} {}", i, j);
        ans[i].push(nn);
        ans[j].push(nn);
        if ans[i].len() == k - 1 {
            i += 1;
            j = i + 1;
        } else {
            j += 1;
        }
    }

    println!("Yes");
    println!("{}", k);
    for a0 in &ans {
        print!("{} ", a0.len());
        for (i, a1) in a0.iter().enumerate() {
            print!("{}", a1);
            if i == a0.len() - 1 {
                println!();
            } else {
                print!(" ");
            }
        }
    }
}
