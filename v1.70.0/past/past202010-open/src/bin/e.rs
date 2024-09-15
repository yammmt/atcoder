use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        _n: usize,
        s: Chars,
    }
    let mut t = s.clone();
    t.sort_unstable();
    while {
        let mut tr = t.clone();
        tr.reverse();

        if t != s && tr != s {
            println!("{}", t.iter().collect::<String>());
            return;
        }

        t.next_permutation()
    } {}

    println!("None");
}
