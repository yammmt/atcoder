use itertools::Itertools;
use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        n: usize,
        sn: [Chars; n],
    }

    for perm in (0..n).permutations(3) {
        let mut one = sn[perm[0]].clone();
        let mut two = sn[perm[1]].clone();
        let mut three = sn[perm[2]].clone();
        two.append(&mut three);
        one.sort_unstable();
        two.sort_unstable();
        if one == two {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
