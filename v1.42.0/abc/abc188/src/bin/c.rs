use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [i64; 2u64.pow(n as u32) as usize],
    }

    let mut an_front = vec![];
    for i in 0..an.len() / 2 {
        an_front.push((an[i], i + 1));
    }
    let mut an_back = vec![];
    for i in an.len() / 2..an.len() {
        an_back.push((an[i], i + 1));
    }
    an_front.sort_unstable();
    an_back.sort_unstable();
    println!(
        "{}",
        if an_front[an_front.len() - 1].0 > an_back[an_back.len() - 1].0 {
            an_back[an_back.len() - 1].1
        } else {
            an_front[an_front.len() - 1].1
        }
    );
}
