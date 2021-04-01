use proconio::input;

fn main() {
    input! {
        t: usize,
        ct: [u64; t],
    }
    for c in &ct {
        println!(
            "{}",
            if *c % 2 == 1 {
                "Odd"
            } else if *c % 4 == 0 {
                "Even"
            } else {
                "Same"
            }
        );
    }
}
