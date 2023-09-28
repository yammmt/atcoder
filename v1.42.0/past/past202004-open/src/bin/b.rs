use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut a_cnt = 0;
    let mut b_cnt = 0;
    let mut c_cnt = 0;
    for c in &s {
        match *c {
            'a' => a_cnt += 1,
            'b' => b_cnt += 1,
            'c' => c_cnt += 1,
            _ => unreachable!(),
        }
    }

    println!(
        "{}",
        if a_cnt > b_cnt && a_cnt > c_cnt {
            "a"
        } else if b_cnt > a_cnt && b_cnt > c_cnt {
            "b"
        } else {
            "c"
        }
    );
}
