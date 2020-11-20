// 5min

use proconio::input;

fn main() {
    input! {
        n: usize,
        sn: [u32; n],
    }

    let snsum = sn.iter().sum::<u32>();
    if snsum % 10 != 0 {
        println!("{}", snsum);
        return;
    }

    let mut minminus = snsum;
    for s in &sn {
        if *s % 10 == 0 {
            continue;
        }

        minminus = minminus.min(*s);
    }
    println!("{}", snsum - minminus);
}
