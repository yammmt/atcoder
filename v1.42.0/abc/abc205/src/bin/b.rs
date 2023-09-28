use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [usize; n],
    }

    let mut exists = vec![false; n];
    for a in &an {
        if exists[*a - 1] {
            println!("No");
            return;
        }

        exists[*a - 1] = true;
    }

    println!("Yes");
}
