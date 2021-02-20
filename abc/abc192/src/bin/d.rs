// X が max 60 桁
// X = 1 だと進数関係なく全部満たすのでは？

use proconio::input;

// X に合わせるよう 10 進数表記 M を base 進数表記にする
// 二つのベクタのサイズで判定 -> サイズ同じなら桁大きい方から大小判定する
fn is_m_base_n_pass(mut m: u128, x: &Vec<char>, base: u128) -> bool {
    println!("m: {:?}, x: {:?}", m, x);
    if m == 1 {
        let v = vec!['1'];
        return !is_x_greater_than_m(&v, &x);
    }

    let mut new_m = vec![];
    while m > 0 {
        // 10 超えたら適当に ASCII で後ろの文字にしときゃ今回の大小判定は通る
        new_m.push(std::char::from_digit((m % base) as u32, 10).unwrap_or('A'));
        m /= base;
    }

    !is_x_greater_than_m(&new_m, &x)
}

// 小さい順に値を格納されている
fn is_x_greater_than_m(m: &Vec<char>, x: &Vec<char>) -> bool {
    println!("m: {:?}, x: {:?}", m, x);
    if x.len() > m.len() {
        true
    } else if x.len() < m.len() {
        false
    } else {
        for i in (0..m.len()).rev() {
            if x[i] > m[i] {
                return true;
            } else if x[i] < m[i] {
                return false;
            }
        }
        // same
        false
    }
}

fn main() {
    input! {
        cx: String,
        m: u128,
    }

    let x = cx.chars().collect::<Vec<char>>();

    let mut vx = vec![];
    for i in 0..x.len() {
        vx.push(x[i].to_digit(10).unwrap() as u128);
    }
    vx.sort_unstable();
    let d = vx[vx.len() - 1] as u128;

    // d + 1 桁時点で M より大きければもう終わり
    if !is_m_base_n_pass(m, &x, d + 1) {
        println!("0");
        return;
    }
    println!("pass");

    let mut fail = std::u64::MAX as u128;
    let mut pass = d + 1;
    while fail - pass > 1 {
        let mid = (fail + pass) / 2;
        if is_m_base_n_pass(m, &x, mid) {
            pass = mid;
        } else {
            fail = mid;
        }
    }

    println!("{}", fail - (d + 1));
}
