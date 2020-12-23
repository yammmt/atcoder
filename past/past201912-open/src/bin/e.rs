use proconio::input;
// use proconio::marker::Chars;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use permutohedron::heap_recursive;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut follows = vec![vec!["N"; n]; n];
    // let mut followers = vec![HashSet::new(); n];
    for _ in 0..q {
        input! {
            t: usize,
        }
        match t {
            1 => {
                input! {
                    a: usize,
                    b: usize,
                }
                follows[a - 1][b - 1] = "Y";
            },
            2 => {
                input! {
                    a: usize,
                }
                for i in 0..n {
                    if follows[i][a - 1] == "Y" {
                        follows[a - 1][i] = "Y";
                    }
                }
            },
            // 3
            _ => {
                input! {
                    a: usize,
                }
                let mut ffs = vec![];
                for i in 0..n {
                    // println!("follows[{}]: {:?}", a-1, follows[a-1]);
                    if follows[a - 1][i] == "Y" {
                        ffs.push(i);
                    }
                    // println!("{} follows {}", a - 1, i);
                }
                for i in 0..ffs.len() {
                    for j in 0..n {
                        if j != a - 1 && follows[ffs[i]][j] == "Y" {
                            // println!("  {} follows {}", i, j);
                            follows[a - 1][j] = "Y";
                        }
                    }
                }
            }
        }
        // println!("{:?}", follows);
    }
    for i in 0..n {
        for j in 0..n {
            print!("{}", follows[i][j]);
        }
        println!();
    }
}
