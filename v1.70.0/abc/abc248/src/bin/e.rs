use proconio::fastout;
use proconio::input;
use std::collections::HashSet;

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        xyn: [(i64, i64); n],
    }

    // k = 1 は明らかに無限
    if k == 1 {
        println!("Infinity");
        return;
    }

    // 2 点で方程式を立て, 残りの全点に対し通るかを判定して数え上げる
    let mut a_and_b = HashSet::new();
    let mut x_same = HashSet::new();
    for i in 0..n {
        let x1 = xyn[i].0;
        let y1 = xyn[i].1;
        for j in i + 1..n {
            let x2 = xyn[j].0;
            let y2 = xyn[j].1;

            let mut passed_cnt = 2;
            for k in j + 1..n {
                let x = xyn[k].0;
                let y = xyn[k].1;

                let left = (y - y1) * (x2 - x1);
                let right = (y2 - y1) * (x - x1);
                if left == right {
                    passed_cnt += 1;
                }
            }

            if passed_cnt >= k {
                // 方程式が被る場合がある
                if x2 - x1 == 0 {
                    x_same.insert(x2);
                } else {
                    // 小数で死ぬので分数表記にして約分する
                    let a_child = y2 - y1;
                    let a_mother = x2 - x1;
                    let a_gcd = gcd(a_child.abs(), a_mother.abs());
                    // (分子, 分母)
                    let a = if a_child.signum() * a_mother.signum() >= 0 {
                        (a_child.abs() / a_gcd, a_mother.abs() / a_gcd)
                    } else {
                        (-(a_child.abs() / a_gcd), a_mother.abs() / a_gcd)
                    };

                    let b_child = x2 * y1 - x1 * y2;
                    let b_mother = x2 - x1;
                    let b_gcd = gcd(b_child.abs(), b_mother.abs());
                    let b = if b_child.signum() * b_mother.signum() >= 0 {
                        (b_child.abs() / b_gcd, b_mother.abs() / b_gcd)
                    } else {
                        (-(b_child.abs() / b_gcd), b_mother.abs() / b_gcd)
                    };

                    a_and_b.insert((a, b));
                }
            }
        }
    }

    println!("{}", x_same.len() + a_and_b.len());
}
