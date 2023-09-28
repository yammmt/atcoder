use proconio::input;

fn main() {
    input! {
        mut n: usize,
    }

    if n == 0 {
        println!("0");
        return;
    }

    let mut ans = vec![];
    while n > 0 {
        let cur = (n % 36) as u8;
        ans.push(if cur < 10 {
            (b'0' + cur) as char
        } else {
            (b'A' + cur - 10) as char
        });
        n /= 36;
    }

    ans.reverse();
    println!("{}", ans.iter().collect::<String>());
}
