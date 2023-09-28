use proconio::input;

fn main() {
    input! {
        mut n: usize,
    }

    let mut ans = vec![];
    while n > 0 {
        if n % 2 == 0 && n != 2 {
            ans.push('B');
            n /= 2;
        } else {
            ans.push('A');
            n -= 1;
        }
    }
    ans.reverse();

    for a in &ans {
        print!("{}", a);
    }
    println!();
}
