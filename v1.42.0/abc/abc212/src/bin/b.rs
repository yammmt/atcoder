// WA: 誤読

use proconio::input;
use proconio::marker::Bytes;

fn main() {
    input! {
        x4: Bytes,
    }
    if x4.iter().all(|c| *c == x4[0]) {
        println!("Weak");
        return;
    }
    let mut is_strong = false;
    for i in 0..3 {
        let next = if x4[i] <= b'8' {
            x4[i] + 1
        } else {
            b'0'
        };

        if x4[i + 1] != next {
            is_strong = true;
            break;
        }
    }

    println!(
        "{}",
        if is_strong {
            "Strong"
        } else {
            "Weak"
        }
    );
}
