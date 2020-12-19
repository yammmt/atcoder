use proconio::input;
// use proconio::marker::Chars;
// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use permutohedron::heap_recursive;

fn main() {
    input! {
        h: usize,
        w: usize,
        ahw: [[u32; w]; h],
    }
    let mut mina = 101;
    for i in 0..h {
        for j in 0..w {
            mina = mina.min(ahw[i][j]);
        }
    }
    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            ans += ahw[i][j] - mina;
        }
    }
    println!("{}", ans);
}
