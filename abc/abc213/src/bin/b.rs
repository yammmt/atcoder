use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [i64; n],
    }

    let mut ain = vec![];
    for (i, a) in an.iter().enumerate() {
        ain.push((*a, i + 1));
    }
    ain.sort_unstable();
    ain.reverse();
    println!("{}", ain[1].1);
}
