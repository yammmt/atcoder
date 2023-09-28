use proconio::input;
use std::cmp::Ordering;

fn digit_vec(mut n: u64) -> Vec<u64> {
    let mut ret = vec![];
    while n > 0 {
        ret.push(n % 10);
        n /= 10;
    }
    ret
}

#[allow(clippy::comparison_chain)]
fn main() {
    input! {
        bb: [u64; 10],
        n: usize,
        mut an: [u64; n],
    }
    let mut vb = vec![0; 10];
    for (i, b) in bb.iter().enumerate() {
        vb[*b as usize] = i;
    }
    // println!("{:?}", vb);

    an.sort_by(|a, b| {
        let digit_a = digit_vec(*a);
        let digit_b = digit_vec(*b);
        match digit_a.len().cmp(&(digit_b.len())) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => match digit_a.cmp(&digit_b) {
                Ordering::Equal => Ordering::Equal,
                _ => {
                    for i in (0..digit_a.len()).rev() {
                        let vba = vb[digit_a[i] as usize];
                        let vbb = vb[digit_b[i] as usize];
                        if vba > vbb {
                            return Ordering::Greater;
                        } else if vba < vbb {
                            return Ordering::Less;
                        }
                    }
                    unreachable!();
                }
            },
        }
    });

    for a in &an {
        println!("{}", a);
    }
}
