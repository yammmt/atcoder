use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        an: [usize; n],
    }

    let mut numbers = vec![];
    for a in &an {
        numbers.push(*a);
    }
    numbers.sort();

    for a in &an {
        println!(
            "{}",
            if *a < numbers[k % n] {
                k / n + 1
            } else {
                k / n
            }
        );
    }
}
