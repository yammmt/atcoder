// :fu: 21-03

use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        mut w: i64,
    }
    w *= 1000;

    let mut i = 1;
    let mut saisho = std::i64::MAX / 2;
    let mut saidai = 0;
    loop {
        // println!("{}", i);
        let cur_l = a * i;
        let cur_h = b * i;
        // println!("{} {}", cur_l, cur_h);
        if cur_h == w || (cur_h > w && cur_h - (b - a) * i <= w) {
            saisho = saisho.min(i);
        }
        if cur_l == w || (cur_l < w && cur_l + (b - a) * i >= w) {
            saidai = i;
        }
        if cur_l > w {
            if saidai == 0 {
                println!("UNSATISFIABLE");
            } else {
                println!("{} {}", saisho, saidai);
            }
            return;
        }
        i += 1;
    }
}
