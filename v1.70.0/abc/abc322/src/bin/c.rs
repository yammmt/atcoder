use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        am: [usize; m],
    }

    let mut a_idx = 0;
    for i in 0..n {
        if i > am[a_idx] - 1 {
            // am[a_idx] < am[a_idx + 1]
            a_idx += 1;
        }
        println!("{}", am[a_idx] - (i + 1));
    }
}
