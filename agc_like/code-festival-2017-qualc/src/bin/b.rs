// :fu:
// 11min

use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [i64; n],
    }
    let mut odd_num = vec![];
    for a in &an {
        if *a % 2 == 0 {
            odd_num.push(2);
        } else {
            odd_num.push(1);
        }
    }


    println!(
        "{}",
        3u32.pow(n as u32) - odd_num.iter().product::<u32>()
    );
}
