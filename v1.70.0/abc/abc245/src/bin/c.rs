use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i32,
        an: [i32; n],
        bn: [i32; n],
    }

    let mut a_available = true;
    let mut b_available = true;
    for i in 1..n {
        let mut a_pass = false;
        let mut b_pass = false;

        if a_available {
            if (an[i] - an[i - 1]).abs() <= k {
                a_pass = true;
            }
            if (bn[i] - an[i - 1]).abs() <= k {
                b_pass = true;
            }
        }
        if b_available {
            if (an[i] - bn[i - 1]).abs() <= k {
                a_pass = true;
            }
            if (bn[i] - bn[i - 1]).abs() <= k {
                b_pass = true;
            }
        }

        if !a_pass && !b_pass {
            println!("No");
            return;
        }

        a_available = a_pass;
        b_available = b_pass;
    }

    println!("Yes");
}
