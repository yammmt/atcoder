// WA: 検索範囲でバカ

use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    }

    let mut cur = b;
    while cur > 0 {
        let y = b / cur;
        if y == 0 {
            cur -= 1;
            continue;
        }

        let x = y - 1;
        let xx = x * cur;
        let yy = y * cur;
        if a <= xx && yy <= b * cur && xx < yy {
            println!("{}", cur);
            return;
        }
        cur -= 1;
    }
}
