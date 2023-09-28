// 32min 数問？

use proconio::input;

fn to_num(v: Vec<i64>) -> i64 {
    let mut ret = 0;
    for vv in &v {
        ret *= 10;
        ret += *vv;
    }
    ret
}

fn main() {
    input! {
        x: i64,
    }

    let mut ans = std::i64::MAX;
    // 最下位桁を a とする
    for a in 0..10 {
        // println!("a: {}", a);
        // 公差を b とする
        for b in -9..10 {
            // println!("  b: {}", b);
            if a == 0 && b == 0 {
                continue;
            }

            let mut vcur = vec![a];
            loop {
                let mut vcur_rev = vcur.clone();
                vcur_rev.reverse();
                let cur_num = to_num(vcur_rev);
                if cur_num >= x {
                    // println!("    {}", cur_num);
                    ans = ans.min(cur_num);
                    break;
                }

                let next = *vcur.last().unwrap() + b;
                if !(0..10).contains(&next) {
                    break;
                }

                vcur.push(next);
            }
        }
    }

    println!("{}", ans);
}
