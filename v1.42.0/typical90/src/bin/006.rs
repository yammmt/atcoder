// :fu: 21-04 遅い

use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }

    let mut abc = vec![VecDeque::new(); 26];
    for (i, c) in s.iter().enumerate() {
        abc[(*c as u8 - b'a') as usize].push_back(i);
    }

    let mut ans = vec![];
    let mut last_i = -1;
    for i in 0..k {
        // println!("i: {}", i);
        // println!("{:?}", abc);
        let available_i = n - k + i;
        'j_loop: for j in 0..26 {
            // println!("  {}", j);
            while let Some(cur_i) = abc[j].pop_front() {
                if cur_i as isize > last_i && cur_i <= available_i {
                    // println!("  {}", (b'a' + j as u8) as char);
                    ans.push((b'a' + j as u8) as char);
                    last_i = cur_i as isize;
                    break 'j_loop;
                } else if cur_i > available_i {
                    abc[j].push_front(cur_i);
                    break;
                }
            }
        }
    }

    println!("{}", ans.iter().collect::<String>());
}
