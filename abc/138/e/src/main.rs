// -*- coding:utf-8-unix -*-

// 愚直なシミュレーションは aaaa...z から zzz... を作ると TLE

use proconio::input;

// v[i] > k を満たす最小の i を返す
// 見つからなければ `v.len()` を返す
fn upper_bound(v: &[isize], k: isize) -> usize {
    let mut low: isize = -1;
    let mut high = v.len() as isize;

    while high - low > 1 {
        let mid = (low + high) / 2;
        if v[mid as usize] > k {
            high = mid;
        } else {
            low = mid;
        }
    }
    high as usize
}

fn main() {
    input! {
        s: String,
        t: String,
    }

    let vs = s.chars().collect::<Vec<char>>();
    let vt = t.chars().collect::<Vec<char>>();
    let mut vsappear = vec![vec![]; 26];
    for i in 0..vs.len() {
        vsappear[(vs[i] as u8 - b'a') as usize].push(i as isize);
    }
    for c in &vt {
        if vsappear[(*c as u8 - b'a') as usize].is_empty() {
            // 冗長な判定だが O(vt.len())
            println!("-1");
            return;
        }
    }

    let mut ans = 0u64;
    let mut last_idx = -1isize;
    for c in &vt {
        // println!("c: {}", c);
        let c_as_usize = (*c as u8 - b'a') as usize;
        let mut cidx = upper_bound(&vsappear[c_as_usize][..], last_idx) as isize;
        if cidx == vsappear[c_as_usize].len() as isize {
            // loop
            // println!("loop");
            ans += (vs.len() as isize - last_idx) as u64;
            cidx = 0;
            last_idx = 0;
        }
        ans += (vsappear[c_as_usize][cidx as usize] as isize - last_idx) as u64;
        last_idx = vsappear[c_as_usize][cidx as usize] as isize;
        // println!("ans: {}", ans);
    }

    println!("{}", ans);
}
