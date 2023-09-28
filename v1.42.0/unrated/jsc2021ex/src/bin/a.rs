use proconio::input;

fn main() {
    input! {
        a: char,
        b: char,
    }
    let an = a.to_digit(10).unwrap();
    let bn = b.to_digit(10).unwrap();
    let mut aa = vec![];
    let mut bb = vec![];
    for _ in 0..bn {
        aa.push(a);
    }
    for _ in 0..an {
        bb.push(b);
    }
    let aas = aa.iter().collect::<String>();
    let bbs = bb.iter().collect::<String>();

    println!(
        "{}",
        if aas < bbs {
            aas
        } else {
            bbs
        }
    );
}
