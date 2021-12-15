use proconio::input;

fn main() {
    input! {
        n: u64,
    }
    let d = 1_000_000_007;

    // すべての数列の個数: 10^N
    // 0/9 を **どちらも** 含まない数列の個数: 8^N
    // 0/9 のどちらか片方のみを含む数列の個数: 9^N - 8^N
    // ans: 10^N - 8^N - 2*(9^N-8^N)
    let mut ten_n = 1u64;
    let mut eight_n = 1u64;
    let mut nine_n = 1u64;
    for _ in 0..n {
        ten_n = (ten_n * 10) % d;
        eight_n = (eight_n * 8) % d;
        nine_n = (nine_n * 9) % d;
    }
    let mut ans = ten_n;
    ans = ((ans + d) - eight_n) % d;
    ans = ((ans + d) - (nine_n + d - eight_n) % d) % d;
    ans = ((ans + d) - (nine_n + d - eight_n) % d) % d;

    println!("{}", ans);
}
