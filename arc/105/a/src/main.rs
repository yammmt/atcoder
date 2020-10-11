// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        abcd: [usize; 4],
    }
    // let mut asum = abcd.iter().sum::<usize>();
    // let mut bsum = 0;
    let n = 4;
    for bit_row in 0..2u32.pow(n) {
        let mut selected = vec![];
        for i in 0..n {
            if bit_row & (1 << i) > 0 {
                selected.push(i);
            }
        }
        // println!("{:?}", selected);
        let mut asum = 0;
        let mut bsum = 0;
        for i in 0..4 {
            if selected.contains(&i) {
                asum += abcd[i as usize];
            } else {
                bsum += abcd[i as usize];
            }
        }
        if asum == bsum {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
