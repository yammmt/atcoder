// 35min (binary search)

use proconio::input;

fn main() {
    input! {
        n: u64,
    }

    let mut m = 2;
    let mut csum = 1;
    while csum < n {
        csum += m;
        m += 1;
    }

    for i in 1..m {
        if i == csum - n {
            continue;
        }

        println!("{}", i);
    }
}
