use proconio::input;

fn main() {
    input! {
        mut n: u64,
    }

    let n_org = n;
    let mut prime_num = 0;
    let mut divisor = 2;
    while n > 1 && divisor * divisor <= n_org {
        if n % divisor == 0 {
            prime_num += 1;
            n /= divisor;
        } else if divisor * divisor > n {
            prime_num += 1;
            break;
        } else {
            divisor += 1;
        }
    }
    // println!("{}", prime_num);

    let mut ans = 0;
    let mut cur = 1;
    while cur < prime_num {
        ans += 1;
        cur *= 2;
    }

    println!("{}", ans);
}
