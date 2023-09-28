use proconio::input;

fn main() {
    input! {
        mut abc: [usize; 3],
    }
    abc.sort();
    println!("{}", abc[1] + abc[2]);
}
