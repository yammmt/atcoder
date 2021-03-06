use proconio::input;

fn main() {
    input! {
        n: usize,
        abn: [(i64, i64); n],
    }

    let mut an = vec![];
    let mut bn = vec![];
    for (i, ab) in abn.iter().enumerate() {
        an.push((ab.0, i));
        bn.push((ab.1, i));
    }
    an.sort();
    bn.sort();

    let mut anss = vec![];
    if an[0].1 == bn[0].1 {
        anss.push(an[0].0 + bn[0].0);
        anss.push((an[0].0).max(bn[1].0));
        anss.push((bn[0].0).max(an[1].0));
    } else {
        anss.push((an[0].0).max(bn[0].0));
    }
    anss.sort();

    println!("{}", anss[0]);
}
