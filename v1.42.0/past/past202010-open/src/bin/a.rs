use proconio::input;

fn main() {
    input! {
        abc: [usize; 3],
    }
    let mut abc2 = vec![(abc[0], 'A'), (abc[1], 'B'), (abc[2], 'C')];
    abc2.sort();
    println!("{}", abc2[1].1);
}
