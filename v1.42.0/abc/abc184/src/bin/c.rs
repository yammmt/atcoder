// 本番中の AC は嘘解で after_contest が追加された
// 嘘解法では例えば入力 1 1 1 6 で 3 になってしまう (マンハッタン距離 5 であり移動二度で辿り着ける)

use proconio::input;

fn main() {
    input! {
        rc1: (i64, i64),
        rc2: (i64, i64),
    }

    if rc1 == rc2 {
        println!("0");
    } else if rc1.0 + rc1.1 == rc2.0 + rc2.1 || rc1.0 - rc1.1 == rc2.0 - rc2.1 || (rc1.0 - rc2.0).abs() + (rc1.1 - rc2.1).abs() <= 3 {
        println!("1");
    } else{
        let xdiff = (rc1.0 - rc2.0).abs();
        let ydiff = (rc1.1 - rc2.1).abs();
        let mandiff = xdiff + ydiff;
        // println!("{}", xdiff);
        // println!("{}", ydiff);
        if xdiff + ydiff <= 6 || (xdiff - ydiff).abs() <= 3 || mandiff % 2 == 0 {
            println!("2");
        } else {
            println!("3");
        }
    }
}
