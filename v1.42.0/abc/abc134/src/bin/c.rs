use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [u32; n],
    }

    let mut maxa = 0;
    let mut semimaxa = 0;
    for a in &an {
        if *a >= maxa {
            semimaxa = maxa;
            maxa = *a;
        } else if *a > semimaxa {
            semimaxa = *a;
        }
    }

    for a in &an {
        if *a == maxa {
            println!("{}", semimaxa);
        } else {
            println!("{}", maxa);
        }
    }
}
