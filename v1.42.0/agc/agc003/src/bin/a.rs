// 3min

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let contains_n = s.iter().any(|a| *a == 'N');
    let contains_w = s.iter().any(|a| *a == 'W');
    let contains_s = s.iter().any(|a| *a == 'S');
    let contains_e = s.iter().any(|a| *a == 'E');

    if (contains_n && !contains_s)
        || (!contains_n && contains_s)
        || (contains_w && !contains_e)
        || (!contains_w && contains_e)
    {
        println!("No");
    } else {
        println!("Yes");
    }
}
