use proconio::input;

fn main() {
    input! {
        abcde: [i64; 5],
    }

    let mut sums = vec![];
    for a in 0..5 {
        for b in a + 1..5 {
            for c in b + 1..5 {
                sums.push(abcde[a] + abcde[b] + abcde[c]);
            }
        }
    }
    sums.sort_unstable();
    sums.reverse();

    println!("{}", sums[2]);
}
