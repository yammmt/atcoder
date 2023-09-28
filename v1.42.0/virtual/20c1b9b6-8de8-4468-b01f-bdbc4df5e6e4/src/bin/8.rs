use proconio::input;

fn main() {
    input! {
        mut n: usize,
    }
    let mut ans = vec![];
    while n > 0 {
        ans.push(n % 10);
        n /= 10;
    }
    while ans.len() < 4 {
        ans.push(0);
    }
    ans.reverse();
    for a in &ans {
        print!("{}", a);
    }
    println!();
}
