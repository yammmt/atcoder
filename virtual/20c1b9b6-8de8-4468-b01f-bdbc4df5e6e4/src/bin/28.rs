// 嫌

use proconio::input;

fn main() {
    input! {
        mut n: u64,
    }

    let mut ans = vec![];
    while n > 0 {
        if n % 2 == 0 {
            n /= 2;
            ans.push('B');
        } else {
            n -= 1;
            ans.push('A');
        }
    }
    ans.reverse();
    println!("{}", ans.iter().collect::<String>());
}
