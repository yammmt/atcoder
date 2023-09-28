// 21min

use proconio::input;

fn main() {
    input! {
        k: usize,
        a: usize,
        b: usize,
    }


    if k == 1 {
        println!("2");
        return;
    } else if a >= b - 2 || k < (a - 1) + 2 {
        // 延々と叩く
        println!("{}", k + 1);
        return;
    }


    let after_offset = k - (a - 1);
    let yentobis = after_offset / 2;
    let mut ans = yentobis * b - (yentobis - 1) * a;
    if after_offset % 2 == 1 {
        ans += 1;
    }
    println!("{}", ans);
}
