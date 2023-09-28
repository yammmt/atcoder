// -*- coding:utf-8-unix -*-

// https://imulan.hatenablog.jp/entry/2015/10/25/012906

// 別解: https://atcoder.jp/contests/abc030/submissions/17719897
// i 回目の位置が bn[i] ではないことに気付かないとサンプルだけ通ってテストケース全滅した

use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: usize,
        k: String,
    }
    let mut bn = vec![a];
    for _ in 0..n {
        input! {
            b: usize,
        }
        bn.push(b);
    }

    let knum = k.parse::<u64>();
    let is_k_small = match knum {
        Ok(_) => true,
        Err(_) => false,
    };

    let mut hm = HashMap::new();
    let mut cycle = 0;
    let mut offset = 0;
    let mut place = vec![0; n + 1];
    place[0] = a;
    hm.insert(a, 0);
    for i in 0..n + 1 {
        // reach answer before loop
        if is_k_small && Ok((i + 1) as u64) == knum {
            println!("{}", bn[place[i]]);
            return;
        }

        match hm.get(&bn[place[i]]) {
            Some(v) => {
                cycle = (i + 1 - v) as u64;
                offset = *v as u64;
                break;
            },
            None => {
                hm.insert(bn[place[i]], i + 1);
                place[i + 1] = bn[place[i]];
            }
        }
    }
    // println!("{:?}", hm);
    // println!("place: {:?}", place);
    // println!("cycle: {}", cycle);
    // println!("offset: {}", offset);

    // O(digit of k)
    // k - offset
    let mut coffset = offset;
    let mut voffset = vec![];
    while coffset > 0 {
        voffset.push(coffset % 10);
        coffset /= 10;
    }
    let mut vk = k.chars().map(|a| a.to_digit(10).unwrap() as u64).collect::<Vec<_>>();
    for _ in 0..(vk.len() - voffset.len()) {
        voffset.push(0);
    }
    voffset.reverse();
    let mut is_carry = false;
    for i in (0..voffset.len()).rev() {
        if is_carry {
            if vk[i] > 0 && vk[i] - 1 >= voffset[i] {
                vk[i] = vk[i] - 1 - voffset[i];
                is_carry = false;
            } else {
                vk[i] = 10 + vk[i] - 1 - voffset[i];
            }
        } else {
            if vk[i] >= voffset[i] {
                vk[i] = vk[i] - voffset[i];
            } else {
                vk[i] = 10 + vk[i] - voffset[i];
                is_carry = true;
            }
        }
    }
    // println!("{} - {:?} = {:?}", k, voffset, vk);

    // (k - offset) % cycle
    let mut cnum = 0;
    for i in 0..vk.len() {
        cnum = cnum * 10 + vk[i];
        if cnum < cycle {
            continue;
        }

        cnum = cnum % cycle;
    }
    // println!("cycle: {}", cycle);
    // println!("offset: {}", offset);
    // println!("cnum: {}", cnum);

    // (k - offset) % cycle 分進んで ans
    println!("{}", place[(offset + cnum) as usize]);
}
