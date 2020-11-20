// 30min

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let nmmax = n.max(m);
    let nmmin = n.min(m);
    if nmmax == 1 {
        // nmmin == 1
        println!("1");
    } else if nmmin == 1 {
        println!("{}", nmmax - 2);
    } else if nmmin == 2 {
        println!("0");
    } else {
        println!("{}", n * m - ((n + m) * 2 - 4));
    }
}
