use proconio::input;

fn main() {
    input! {
        n: u64,
    }

    if n == 1 {
        println!("1");
        return;
    }

    let mut ans = std::u64::MAX / 2;
    let mut b = 0;
    let mut b2 = 1;
    while b2 < n {
        let a = n / b2;
        let c = n - (a * b2);
        // println!("{} {} {}", a, b, c);
        ans = ans.min(a + b + c);
        b2 *= 2;
        b += 1;
    }

    println!("{}", ans);
}
