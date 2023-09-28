use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [usize; 4*n-1],
    }

    let mut cnt = vec![0; n];
    for a in &an {
        cnt[*a - 1] += 1;
    }

    for (i, c) in cnt.iter().enumerate() {
        if *c != 4 {
            println!("{}", i + 1);
            return;
        }
    }
}
