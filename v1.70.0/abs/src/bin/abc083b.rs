use proconio::input;

fn digit_sum(n: u32) -> u32 {
    let mut m = n;
    let mut ret = 0;
    while m > 0 {
        ret += m % 10;
        m /= 10;
    }
    ret
}

fn main() {
    input! {
        n: usize,
        a: u32,
        b: u32,
    }

    let mut ans = 0;
    for i in 1..=n {
        let cur = digit_sum(i as u32);
        if cur >= a && cur <= b {
            ans += i;
        }
    }

    println!("{ans}");
}
