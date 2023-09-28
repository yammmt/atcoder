// :fu:

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut bn: [usize; n],
    }

    let mut ans = vec![];
    for _ in 0..n {
        let mut removed = None;
        for (i, b) in bn.iter().enumerate() {
            if *b == i + 1 {
                removed = Some(i);
            }
        }
        match removed {
            Some(a) => {
                ans.push(bn[a]);
                bn.remove(a);
            },
            None => {
                println!("-1");
                return;
            }
        }
    }
    ans.reverse();
    for a in &ans {
        println!("{}", a);
    }
}
