// 50min -> x
// :fu: :fu: :fu: 数問

use proconio::input;

fn main() {
    input! {
        n: usize,
        tan: [(u64, u64); n],
    }

    let mut t = 1;
    let mut a = 1;
    for ta in &tan {
        let nt = (t + ta.0 - 1) / ta.0;
        let na = (a + ta.1 - 1) / ta.1;
        let nn = nt.max(na);
        t = nn * ta.0;
        a = nn * ta.1;
    }

    println!("{}", t + a);
}
