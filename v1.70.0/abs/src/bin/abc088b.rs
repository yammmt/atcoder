use proconio::input;

fn main() {
    input! {
        n: usize,
        mut an: [usize; n],
    }

    an.sort_unstable();
    an.reverse();
    let mut alice = 0;
    let mut bob = 0;
    for (i, a) in an.iter().enumerate() {
        if i % 2 == 0 {
            alice += *a;
        } else {
            bob += *a;
        }
    }

    println!("{}", alice - bob);
}
