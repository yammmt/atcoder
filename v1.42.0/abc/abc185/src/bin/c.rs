use proconio::input;

fn ncr(n: u128, k: u128) -> u128 {
    let mut ans = 1;
    for i in 0..k {
        ans *= n - i;
        ans /= i + 1;
    }
    ans
}

fn main() {
    input! {
        l: u128,
    }

    println!("{}", ncr(l - 1, 11));
}
