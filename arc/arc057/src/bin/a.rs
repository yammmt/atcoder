use proconio::input;

static GOAL: u128 = 2_000_000_000_000;

fn main() {
    input! {
        mut a: u128,
        k: u128,
    }
    if k == 0 {
        println!("{}", GOAL - a);
        return;
    }

    let mut ans = 0;
    while a < GOAL {
        a += 1 + k * a;
        ans += 1;
    }
    println!("{}", ans);
}
