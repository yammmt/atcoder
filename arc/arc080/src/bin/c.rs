use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [u64; n],
    }

    let mut x2 = 0;
    let mut x4 = 0;
    for a in &an {
        if *a % 4 == 0 {
            x4 += 1;
        } else if *a % 2 == 0 {
            x2 += 1;
        }
    }
    if x4 + x2 / 2 >= n / 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
