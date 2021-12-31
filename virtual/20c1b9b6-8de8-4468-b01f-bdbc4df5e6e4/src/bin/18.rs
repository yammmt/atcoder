use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut c_appered = false;
    for c in &s {
        match c {
            'C' => c_appered = true,
            'F' => {
                if c_appered {
                    println!("Yes");
                    return;
                }
            }
            _ => {}
        }
    }

    println!("No");
}
