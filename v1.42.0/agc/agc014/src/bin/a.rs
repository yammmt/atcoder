// 15min 1WA (11.5min)
// WA: a == b == c かつ全部奇数であるパターンを見落としていた

use proconio::input;

fn main() {
    input! {
        mut a: u64,
        mut b: u64,
        mut c: u64,
    }

    if a == b && b == c && a % 2 == 0 {
        println!("-1");
        return;
    }

    let mut ans = 0;
    while a % 2 == 0 && b % 2 == 0 && c % 2 == 0 {
        ans += 1;
        let aa = a;
        let bb = b;
        let cc = c;
        a = (bb + cc) / 2;
        b = (cc + aa) / 2;
        c = (aa + bb) / 2;
    }
    println!("{}", ans);
}
