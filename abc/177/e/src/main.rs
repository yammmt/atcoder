// -*- coding:utf-8-unix -*-

use proconio::input;

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    input! {
        n: usize,
        an: [u64; n],
    }

    let mut g = an[0];
    for a in &an {
        g = gcd(g, *a);
    }
    if g != 1 {
        println!("not coprime");
        return;
    }

    let amax = *an.iter().max().unwrap();
    let mut numnum = vec![0; amax as usize + 1];
    for a in &an {
        numnum[*a as usize] += 1;
    }

    let mut idx = 2;
    let mut is_pairwise = true;
    while idx <= amax && is_pairwise {
        let mut j = idx;
        let mut numsum = 0;
        while j <= amax {
            numsum += numnum[j as usize];
            if numsum > 1 {
                is_pairwise = false;
                break;
            }
            j += idx;
        }
        idx += 1;
    }

    println!(
        "{} coprime",
        if is_pairwise {
            "pairwise"
        } else {
            "setwise"
        }
    );
}
