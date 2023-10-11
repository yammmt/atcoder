use proconio::input;

fn main() {
    input! {
        n: usize,
        mut an: [usize; n],
    }
    an.sort_unstable();

    // 最小/最大をなくすと答えが一意に定まらない
    let amin = an.first().unwrap();
    for (i, a) in an.iter().enumerate() {
        if *a != amin + i {
            println!("{}", amin + i);
            return;
        }
    }
}
