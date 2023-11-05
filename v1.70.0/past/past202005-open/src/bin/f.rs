use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;
// use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        n: usize,
        ann: [Chars; n],
    }

    let mut ans = vec![];
    for i in 0..n / 2 {
        let mut hs_front = HashSet::new();
        for a in &ann[i] {
            hs_front.insert(*a);
        }

        let i_back = n - 1 - i;
        let mut hs_back = HashSet::new();
        for a in &ann[i_back] {
            hs_back.insert(*a);
        }

        let mut cur_pass = false;
        for a in &hs_front {
            if hs_back.contains(a) {
                cur_pass = true;
                ans.push(*a);
                break;
            }
        }
        if !cur_pass {
            println!("-1");
            return;
        }
    }

    print!("{}", ans.iter().collect::<String>());
    if n % 2 == 1 {
        // N が奇数だった場合に中央が抜けているので
        print!("{}", ann[n / 2][0]);
    }
    ans.reverse();
    println!("{}", ans.iter().collect::<String>());
}
