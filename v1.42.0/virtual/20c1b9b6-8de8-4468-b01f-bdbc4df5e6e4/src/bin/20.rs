use proconio::input;

fn main() {
    input! {
        b: char,
    }
    println!(
        "{}",
        match b {
            'A' => 'T',
            'T' => 'A',
            'C' => 'G',
            'G' => 'C',
            _ => unreachable!(),
        }
    );
}
