use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        mut w: i64,
    }

    // => g
    w *= 1000;

    let mut ans_min = 999_999_999;
    let mut ans_max = -1;
    let mut i = 1;
    while a * i <= w {
        let aa = a * i;
        let bb = b * i;

        // i 個で作れる範囲は [aa, bb]
        if aa <= w && w <= bb {
            ans_min = ans_min.min(i);
            ans_max = ans_max.max(i);
        }

        i += 1;
    }

    if ans_min == 999_999_999 || ans_max == -1 {
        println!("UNSATISFIABLE");
    } else {
        println!("{} {}", ans_min, ans_max);
    }
}
