use proconio::input;

fn main() {
    input! {
        n: i64,
    }

    // 4, 7, 10, 13, 16 桁以上の数の数
    // n <=10^15
    let mut ans = 0i64;
    ans += (n - 999).max(0);
    ans += (n - 999_999).max(0);
    ans += (n - 999_999_999).max(0);
    ans += (n - 999_999_999_999).max(0);
    ans += (n - 999_999_999_999_999).max(0);

    println!("{ans}");
}
