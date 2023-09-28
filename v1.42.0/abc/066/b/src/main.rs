// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let vc = s.chars().collect::<Vec<char>>();
    let mut total_len = vc.len() - 2;
    loop {
        let front = vc[0..(total_len / 2)].iter().collect::<String>();
        let rear = vc[(total_len / 2)..total_len].iter().collect::<String>();
        if front == rear {
            println!("{}", front.len() + rear.len());
            return;
        }

        total_len -= 2;
    }
}
