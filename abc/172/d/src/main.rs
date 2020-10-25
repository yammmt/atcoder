// -*- coding:utf-8-unix -*-

// https://www.youtube.com/watch?v=v8ppNGf49Nk&feature=youtu.be

use proconio::input;

fn main() {
    input! {
        n: u64,
    }

    let mut vfk = vec![0; (n + 1) as usize];
    for i in 1..n + 1 {
        let mut j = i;
        // ここで for で回して割って判定取ると間に合わなくなる
        // O(N) と O(logN) (調和級数)
        while j < n + 1 {
            vfk[j as usize] += 1;
            j += i;
        }
    }

    let mut ans = 0u64;
    for i in 1..n + 1 {
        ans += i * vfk[i as usize];
    }
    println!("{}", ans);
}
