use proconio::input;
// use proconio::marker::Chars;
// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use permutohedron::heap_recursive;

fn main() {
    input! {
        n: u128,
    }

    let totallen = (n * (1 + n)) / 2;
    let mut kaeru = n + 1;
    let mut kaenai = 0;
    while kaeru - kaenai > 1 {
        let mid = (kaeru + kaenai) / 2;
        // println!("mid: {}", mid);
        if (mid * (2 * (n + 1) + 1 - mid)) / 2 >= totallen {
            kaeru = mid;
        } else {
            kaenai = mid;
        }
    }
    println!("{}", kaeru);
}
