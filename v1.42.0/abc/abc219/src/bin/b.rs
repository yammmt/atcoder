use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s3: [String; 3],
        t: Chars,
    }
    for tt in &t {
        print!(
            "{}",
            match *tt {
                '1' => &s3[0],
                '2' => &s3[1],
                '3' => &s3[2],
                _ => unreachable!(),
            }
        );
    }
    println!();
}
