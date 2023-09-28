use proconio::input;

fn main() {
    input! {
        n: usize,
        mut an: [u32; n],
    }
    an.sort_unstable();
    an.reverse();
    let mut alice = 0;
    let mut bob = 0;
    let mut is_alice_turn = true;
    for a in &an {
        if is_alice_turn {
            alice += *a;
        } else {
            bob += *a;
        }
        is_alice_turn = !is_alice_turn;
    }
    println!("{}", alice - bob);
}
