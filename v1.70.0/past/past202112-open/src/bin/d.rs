use proconio::fastout;
use proconio::input;
use std::cmp::Reverse;

#[fastout]
fn main() {
    input! {
        n: usize,
        an: [usize; n],
        bn: [usize; n],
    }

    let mut students = vec![];
    for i in 0..n {
        // total, math, id
        students.push((Reverse(an[i] + bn[i]), Reverse(an[i]), i + 1));
    }
    students.sort_unstable();

    for (i, s) in students.iter().enumerate() {
        print!("{}", s.2);
        if i == n - 1 {
            println!();
        } else {
            print!(" ");
        }
    }
}
