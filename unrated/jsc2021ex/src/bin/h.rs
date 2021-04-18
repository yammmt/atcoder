use proconio::input;
use proconio::marker::Chars;

#[allow(clippy::collapsible_if)]
fn main() {
    input! {
        nc: Chars,
    }

    let mut vd = vec![];
    for n in &nc {
        vd.push(n.to_digit(10).unwrap() as usize);
    }
    let vdsum = vd.iter().sum::<usize>();
    if vdsum % 3 == 0 {
        println!("0");
        return;
    }

    let mut mods = vec![0; 3];
    for d in &vd {
        mods[d % 3] += 1;
    }
    if vdsum % 3 == 1 {
        if mods[1] > 0 && nc.len() > 1 {
            println!("1");
        } else if mods[2] > 1 && nc.len() > 2 {
            println!("2");
        } else {
            println!("-1");
        }
    } else {
        if mods[2] > 0 && nc.len() > 1 {
            println!("1");
        } else if mods[1] > 1 && nc.len() > 2 {
            println!("2");
        } else {
            println!("-1");
        }
    }
}
