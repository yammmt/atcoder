use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [usize; n],
    }
    let mut v = vec![];
    for (i, a) in an.iter().enumerate() {
        v.push((*a, i + 1));
    }
    v.sort_unstable();
    println!("{}", v[n - 2].1);
}
