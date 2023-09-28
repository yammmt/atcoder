use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }
    let d = 1_000_000_007u64;

    let mut c_num = 0u64;
    let mut h_num = 0u64;
    let mut o_num = 0u64;
    let mut k_num = 0u64;
    let mut u_num = 0u64;
    let mut d_num = 0u64;
    let mut a_num = 0u64;
    let mut ans = 0;
    for c in &s {
        match *c {
            'c' => c_num += 1,
            'h' => h_num = (h_num + c_num) % d,
            'o' => o_num = (o_num + h_num) % d,
            'k' => k_num = (k_num + o_num) % d,
            'u' => u_num = (u_num + k_num) % d,
            'd' => d_num = (d_num + u_num) % d,
            'a' => a_num = (a_num + d_num) % d,
            'i' => ans = (ans + a_num) % d,
            _ => {}
        }
    }

    println!("{}", ans);
}
