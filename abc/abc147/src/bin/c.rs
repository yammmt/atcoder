// 16min

use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut vxy = vec![];
    for _ in 0..n {
        input! {
            a: usize,
            xya: [(usize, usize); a],
        }
        vxy.push(xya);
    }
    // println!("{:?}", vxy);

    let mut ans = 0;
    for bit_row in 0..2u64.pow(n as u32) {
        let mut honests = vec![];
        for i in 0..n {
            if bit_row & (1 << i) > 0 {
                honests.push(i);
            }
        }
        // println!("{:?}", honests);

        let mut pass = true;
        for i in &honests {
            for xy in &vxy[*i] {
                match xy.1 {
                    0 => {
                        // unkind
                        if honests.contains(&(xy.0 - 1)) {
                            pass = false;
                            break;
                        }
                    }
                    1 => {
                        // honest
                        if !honests.contains(&(xy.0 - 1)) {
                            pass = false;
                            break;
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
        if pass {
            // println!("pass");
            ans = ans.max(honests.len());
        }
    }

    println!("{}", ans);
}
