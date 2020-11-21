// 6.5min

use proconio::input;

fn digit_num(mut n: u64) -> u64 {
    let mut ret = 0;
    while n > 0 {
        n /= 10;
        ret += 1;
    }
    ret
}

fn main() {
    input! {
        n: u64,
    }

    let mut ans = 1;
    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            // 可能な限り桁数揃えた場合が最小値になるので
            // 割る数が大きくなればなるほど望ましい (例: 平方根)
            ans = digit_num(n / i);
        }

        i += 1;
    }
    println!("{}", ans);
}
