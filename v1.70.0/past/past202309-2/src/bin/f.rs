use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

static MOD: u64 = 998_244_353;

#[fastout]
fn main() {
    input! {
        mut s: Chars,
    }
    // 番兵
    s.push('*');

    // 最大で S が 10^6 桁まで続く可能性がある
    // MOD で割った値を掛け算する文には範囲におさまる

    let mut is_first_num = true;
    let mut cur = 0;
    let mut ans = 0;
    for c in &s {
        if *c == '*' {
            if is_first_num {
                is_first_num = false;
                ans = cur;
            } else {
                ans = (ans * cur) % MOD;
            }
            cur = 0;
        } else {
            cur = (cur * 10 + c.to_digit(10).unwrap() as u64) % MOD;
        }
    }

    println!("{ans}");
}
