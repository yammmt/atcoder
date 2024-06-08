use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }
    let mut car = vec![];
    for _ in 0..m {
        input! {
            c: usize,
            ac: [Usize1; c],
            r: char,
        }
        car.push((c, ac, r));
    }

    let mut ans = 0;
    for bit in 0..2u32.pow(n as u32) {
        let mut b = bit;
        let mut is_valid = vec![];
        for _ in 0..n {
            if b % 2 == 0 {
                is_valid.push(true);
            } else {
                is_valid.push(false);
            }
            b /= 2;
        }

        let mut passes = true;
        for (_c, a, r) in &car {
            let mut valid_key_num = 0;
            for &aa in a {
                if is_valid[aa] {
                    valid_key_num += 1;
                }
            }

            if (valid_key_num >= k && *r != 'o') || (valid_key_num < k && *r == 'o') {
                passes = false;
                break;
            }
        }

        if passes {
            ans += 1;
        }
    }

    println!("{ans}");
}
