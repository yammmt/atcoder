use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        mut s: Chars,
        q: usize,
        txcq: [(usize, usize, char); q],
    }

    let mut last_lower_turn = 0;
    let mut last_upper_turn = 0;
    let mut updated_turn = vec![0; n];
    for (i, &(t, x, c)) in txcq.iter().enumerate() {
        match t {
            1 => {
                s[x - 1] = c;
                updated_turn[x - 1] = i;
            }
            2 => last_lower_turn = i,
            3 => last_upper_turn = i,
            _ => unreachable!(),
        }
    }

    for i in 0..n {
        print!(
            "{}",
            if last_upper_turn > last_lower_turn && last_upper_turn > updated_turn[i] {
                s[i].to_ascii_uppercase()
            } else if last_lower_turn > last_upper_turn && last_lower_turn > updated_turn[i] {
                s[i].to_ascii_lowercase()
            } else {
                s[i]
            }
        );
    }
    println!();
}
