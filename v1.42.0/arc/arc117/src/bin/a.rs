use proconio::input;
use std::cmp::Ordering;

fn main() {
    input! {
        a: i64,
        b: i64,
    }

    let mut ans = vec![];
    match a.cmp(&b) {
        Ordering::Greater => {
            for i in 1..a + 1 {
                ans.push(i);
            }
            let mut cur = 0;
            for i in b..a + 1 {
                // println!("-= {}", i);
                cur -= i;
            }
            ans.push(cur);
            for i in 1..b {
                ans.push(-i);
            }
        }
        Ordering::Less => {
            for i in 1..b + 1 {
                ans.push(-i);
            }
            let mut cur = 0;
            for i in a..b + 1 {
                // println!("+= {}", i);
                cur += i;
            }
            ans.push(cur);
            for i in 1..a {
                ans.push(i);
            }
        }
        Ordering::Equal => {
            for i in 1..a + 1 {
                ans.push(i);
                ans.push(-i);
            }
        }
    }

    for (i, a) in ans.iter().enumerate() {
        print!("{}", a);
        if i == ans.len() - 1 {
            println!();
        } else {
            print!(" ");
        }
    }
}
