// -*- coding:utf-8-unix -*-

use proconio::input;

fn pcnt(x: u32) -> u32 {
    x.count_ones()
}

fn f(x: u32) -> u32 {
    if x == 0 {
        0
    } else {
        f(x % pcnt(x)) + 1
    }
}

fn main() {
    input! {
        n: u64,
        s: String,
    }

    let x: Vec<u8> = s.chars().map(|c| if c == '0' { 0 } else { 1 }).collect();
    let pc: u32 = x.iter().filter(|&i| *i == 1).count() as u32;
    let mut ans = Vec::with_capacity(n as usize);
    for _i in 0..n {
        ans.push(0);
    }

    for b in 0..2 {
        let mut npc = pc;
        if b == 0 {
            npc += 1;
        } else {
            if npc < 2 {
                continue;
            }

            npc -= 1;
        }
        let mut r0 = 0;
        for i in 0..n {
            r0 = (r0 * 2) % npc;
            r0 += x[i as usize] as u32;
        }
        let mut k = 1;
        for i in (0..n).rev() {
            // `ii` 桁目を見ている際には `2^ii` が求まっている
            if x[i as usize] == b as u8 {
                let mut r = r0;
                if b == 0 {
                    r = (r + k) % npc;
                } else {
                    r = (npc + r - k) % npc;
                }
                ans[i as usize] = f(r) + 1;
            }
            k = (k * 2) % npc;
        }
    }

    for i in 0..n {
        println!("{}", ans[i as usize]);
    }
}
