// :fu: :fu: :fu: 21-05 算数
// 300 点の難易度？ (主観) 茶色下位は嘘

use proconio::input;

fn main() {
    input! {
        t: usize,
        n: usize,
    }

    let mut cycle = vec![];
    let mut prev = (100 + t) * 1 / 100;
    for i in 2..101 {
        let cur = (100 + t) * i / 100;
        if cur - prev == 2 {
            cycle.push(cur - 1);
        }
        prev = cur;
    }
    // println!("{:?}", cycle);

    let p = (n - 1) / cycle.len();
    let r = (n - 1) % cycle.len();
    println!("{}", cycle[r] as usize + (100 + t) * p);
}
