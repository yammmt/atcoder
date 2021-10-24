// C にしては難しく感じたが

use proconio::input;

fn gen_max_num(mut vn: Vec<u64>) -> u64 {
    vn.sort_unstable();
    vn.reverse();

    let mut ret = 0;
    for nn in vn {
        ret *= 10;
        ret += nn;
    }

    ret
}

fn main() {
    input! {
        mut n: u64,
    }

    let mut components = vec![];
    while n > 0 {
        components.push(n % 10);
        n /= 10;
    }

    let mut ans = 0u64;
    // グループの分け方を全探索
    for i in 0..2u64.pow(components.len() as u32) {
        let mut first = vec![];
        let mut second = vec![];

        // グループ分け
        let mut ii = i;
        for c in &components {
            if ii % 2 == 0 {
                first.push(*c);
            } else {
                second.push(*c);
            }
            ii /= 2;
        }

        if first.is_empty() || second.is_empty() {
            continue;
        }

        let cur_f = gen_max_num(first);
        let cur_s = gen_max_num(second);
        ans = ans.max(cur_f * cur_s);
    }

    println!("{}", ans);
}
