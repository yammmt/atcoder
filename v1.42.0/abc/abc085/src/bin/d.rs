// WA: 分数を整数に切り上げる式, 含まれない a の減算 (想定解ではないが十分に間に合った方法)

use proconio::input;

fn main() {
    input! {
        n: usize,
        h: i64,
        abn: [(i64, i64); n],
    }

    let mut an = vec![];
    let mut bn = vec![];
    for ab in &abn {
        an.push(ab.0);
        bn.push(ab.1);
    }
    let maxa = *(an.iter().max().unwrap());
    bn.sort_unstable();

    let mut attack_num = 0;
    let mut damage = 0;
    for b in bn.iter().filter(|i| **i > maxa).rev() {
        damage += b;
        attack_num += 1;
        if damage >= h {
            break;
        }
    }
    if damage < h {
        attack_num += (h - damage) / maxa;
        if (h - damage) % maxa != 0 {
            attack_num += 1;
        }
    }

    println!("{}", attack_num);
}
