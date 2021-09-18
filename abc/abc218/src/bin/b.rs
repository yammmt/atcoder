use proconio::input;

fn main() {
    input! {
        p: [u8; 26],
    }
    let mut ans = vec![];
    for pp in &p {
        ans.push((b'a' + *pp - 1) as char);
    }

    for a in &ans {
        print!("{}", a);
    }
    println!();
}
