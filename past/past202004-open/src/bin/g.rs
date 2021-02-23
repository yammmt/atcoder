use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        q: usize,
    }

    let mut vdq = VecDeque::new();
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                c: char,
                x: i64,
            }
            vdq.push_back((c, x));
        } else {
            input! {
                d: i64,
            }
            let mut deleted = vec![0; 26];
            let mut deleted_sum = 0;
            while !vdq.is_empty() && deleted_sum < d {
                let cur = vdq.pop_front().unwrap();
                if deleted_sum + cur.1 <= d {
                    deleted[(cur.0 as u8 - b'a') as usize] += cur.1;
                    deleted_sum += cur.1
                } else {
                    let required_len = d - deleted_sum;
                    deleted[(cur.0 as u8 - b'a') as usize] += required_len;
                    deleted_sum += required_len;
                    vdq.push_front((cur.0, cur.1 - required_len));
                }
            }

            let mut ans = 0;
            deleted.iter().for_each(|d| ans += d * d);
            println!("{}", ans);
        }
    }
}
