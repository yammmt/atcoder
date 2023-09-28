// :fu: :fu: :fu: 数問

use proconio::input;

fn main() {
    input! {
        b: i64,
        c: i64,
    }

    let odd_m = -b-(c-1)/2;
    let odd_p = -b+(c-1)/2;
    let even_m = b-c/2;
    let even_p = b+(c-2)/2;

    let odd_l = odd_m.min(odd_p);
    let odd_h = odd_m.max(odd_p);
    let even_l = even_m.min(even_p);
    let even_h = even_m.max(even_p);

    if odd_l < even_l {
        if odd_h < even_l {
            println!("{}", odd_h - odd_l + even_h - even_l + 2);
        } else {
            println!("{}", even_h - odd_l + 1);
        }
    } else if even_h < odd_l {
        println!("{}", even_h - even_l + odd_h - odd_l + 2);
    } else {
        println!("{}", odd_h - even_l + 1);
    }
}
