// 典型寄り？

use proconio::input;

const UNDEFINED: i64 = std::i64::MAX / 2;

fn main() {
    input! {
        n: usize,
        cnn: [[i64; n]; n],
    }

    let mut ijmin_idx = (n + 1, n + 1);
    let mut ijmin_v = UNDEFINED;
    for i in 0..n {
        for j in 0..n {
            if cnn[i][j] < ijmin_v {
                ijmin_idx = (i, j);
                ijmin_v = cnn[i][j];
            }
        }
    }

    let mut an = vec![UNDEFINED; n];
    let bn = cnn[ijmin_idx.0].clone();
    // println!("{:?}", bn);
    for i in 0..n {
        // println!("i: {}", i);
        an[i] = cnn[i][0] - bn[0];
        // println!("  an: {}", an[i]);
        if an[i] < 0 {
            println!("No");
            return;
        }

        for j in 0..n {
            if cnn[i][j] != an[i] + bn[j] {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
    for (i, a) in an.iter().enumerate() {
        print!("{}", a);
        if i == n - 1 {
            println!();
        } else {
            print!(" ");
        }
    }
    for (i, b) in bn.iter().enumerate() {
        print!("{}", b);
        if i == n - 1 {
            println!();
        } else {
            print!(" ");
        }
    }
}
