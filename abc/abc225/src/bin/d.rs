// :fu: :fu: :fu: 21-11 茶色は大嘘

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut front = vec![None; n];
    let mut rear = vec![None; n];
    for _ in 0..q {
        input! {
            a: usize,
        }
        match a {
            1 => {
                input! {
                    x: usize,
                    y: usize,
                }
                rear[x - 1] = Some(y - 1);
                front[y - 1] = Some(x - 1);
            }
            2 => {
                input! {
                    x: usize,
                    y: usize,
                }
                rear[x - 1] = None;
                front[y - 1] = None;
            }
            3 => {
                input! {
                    x: usize,
                }
                let mut ans = vec![];
                ans.push(x);
                let mut cur_i = x - 1;
                while let Some(cur) = front[cur_i] {
                    ans.push(cur + 1);
                    cur_i = cur;
                }
                ans.reverse();

                cur_i = x - 1;
                while let Some(cur) = rear[cur_i] {
                    ans.push(cur + 1);
                    cur_i = cur;
                }

                print!("{} ", ans.len());
                for (i, a) in ans.iter().enumerate() {
                    print!("{}", a);
                    if i == ans.len() - 1 {
                        println!();
                    } else {
                        print!(" ");
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}
