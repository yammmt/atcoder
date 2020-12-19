// :fu:

use proconio::input;

fn main() {
    input! {
        mut n: i64,
    }

    if n == 0 {
        println!("0");
        return;
    }

    let mut ans = vec![];
    let mut dir = -1;
    while n != 0 {
        if n % 2 != 0 {
            ans.push('1');
            n += dir;
        } else {
            ans.push('0');
        }

        n /= 2;
        dir *= -1;
    }

    ans.reverse();
    println!("{}", ans.iter().collect::<String>());
}
