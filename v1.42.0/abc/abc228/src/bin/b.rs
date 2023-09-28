use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        an: [usize; n],
    }

    let mut know = vec![false; n];
    let mut cur = x - 1;
    while !know[cur] {
        know[cur] = true;
        cur = an[cur] - 1;
    }

    println!("{}", know.iter().filter(|k| **k).count());
}
