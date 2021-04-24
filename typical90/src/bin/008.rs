use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }
    let d = 1_000_000_007u64;

    let mut a_num = 0;
    let mut t_num = 0;
    let mut c_num = 0;
    let mut o_num = 0;
    let mut d_num = 0;
    let mut e_num = 0;
    let mut r_num = 0;
    for c in &s {
        match *c {
            'a' => a_num = (a_num + 1) % d,
            't' => t_num = (t_num + a_num) % d,
            'c' => c_num = (c_num + t_num) % d,
            'o' => o_num = (o_num + c_num) % d,
            'd' => d_num = (d_num + o_num) % d,
            'e' => e_num = (e_num + d_num) % d,
            'r' => r_num = (r_num + e_num) % d,
            _ => {}
        }
    }

    println!("{}", r_num);
}
