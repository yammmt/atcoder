use proconio::input;
use proconio::marker::Bytes;

fn main() {
    input! {
        s: Bytes,
        t: Bytes,
    }

    let ss: Vec<u8> = s.iter().map(|&c| c - b'a').collect();

    for k in 0..26 {
        let tt: Vec<u8> = t.iter().map(|&c| (c - b'a' + k) % 26).collect();
        if tt == ss {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
