use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
    }
    let mut van = vec![vec![0; n]; n];
    for i in 0..n {
        for j in i + 1..n {
            input! {
                a: i64,
            }
            van[i][j] = a;
            van[j][i] = a;
        }
    }
    // println!("{:?}", van);

    let mut vdq = VecDeque::new();
    vdq.push_back(((vec![0], vec![], vec![]), 1));
    let mut ans = std::i64::MIN;
    while let Some(cur) = vdq.pop_back() {
        if cur.1 == n {
            // println!("{:?}", cur.0);
            let mut pts = 0;
            let cur00 = (cur.0).0;
            let cur01 = (cur.0).1;
            let cur02 = (cur.0).2;
            // println!("grp 0");
            for i in 0..cur00.len() {
                for j in i + 1..cur00.len() {
                    // println!("{} {}: {}", i, j, van[i][j]);
                    pts += van[cur00[i]][cur00[j]];
                }
            }
            // println!("grp 1");
            for i in 0..cur01.len() {
                for j in i + 1..cur01.len() {
                    // println!("{} {}: {}", i, j, van[i][j]);
                    pts += van[cur01[i] as usize][cur01[j]];
                }
            }
            // println!("grp 2");
            for i in 0..cur02.len() {
                for j in i + 1..cur02.len() {
                    // println!("{} {}: {}", i, j, van[i][j]);
                    pts += van[cur02[i] as usize][cur02[j]];
                }
            }
            // println!("pts: {}", pts);
            ans = ans.max(pts);
            continue;
        }

        let cur00 = (cur.0).0.clone();
        let cur01 = (cur.0).1.clone();
        let cur02 = (cur.0).2.clone();
        let mut cur0add = (cur.0).0.clone();
        cur0add.push(cur.1);
        vdq.push_back(((cur0add, cur01.clone(), cur02.clone()), cur.1 + 1));
        let mut cur1add = (cur.0).1.clone();
        cur1add.push(cur.1);
        vdq.push_back(((cur00.clone(), cur1add, cur02.clone()), cur.1 + 1));
        let mut cur2add = (cur.0).2.clone();
        cur2add.push(cur.1);
        vdq.push_back(((cur00.clone(), cur01.clone(), cur2add), cur.1 + 1));
    }

    println!("{}", ans);
}
