use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;

fn main() {
    input! {
        s: Chars,
    }

    let o_num = s.iter().filter(|&c| *c == 'o').count();
    if o_num > 4 {
        println!("0");
        return;
    }

    let mut ans = 0;
    for i in 0..10000 {
        let mut cur_i = i;
        let mut cur_num = vec![];
        for _ in 0..4 {
            cur_num.push(cur_i % 10);
            cur_i /= 10;
        }

        let mut included_o = HashSet::new();
        for n in &cur_num {
            if s[*n as usize] == 'o' {
                included_o.insert(*n);
            } else if s[*n as usize] == 'x' {
                // 雑
                included_o.insert(0);
                included_o.insert(1);
                included_o.insert(2);
                included_o.insert(3);
                included_o.insert(4);
            }
        }

        if included_o.len() == o_num {
            ans += 1;
        }
    }

    println!("{}", ans);
}
