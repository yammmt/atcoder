use proconio::input;

fn to_6digit_chars(mut n: usize) -> Vec<char> {
    let mut ret = vec![];
    for _ in 0..6 {
        ret.push(char::from_digit(n as u32 % 10, 10).unwrap());
        n /= 10;
    }
    ret.reverse();
    ret
}

fn main() {
    input! {
        n: usize,
        m: usize,
        pym: [(usize, usize); m],
    }

    let mut v = vec![vec![]; n + 1];
    for (i, py) in pym.iter().enumerate() {
        v[py.0].push((py.1, i));
    }

    let mut ans = vec![String::new(); m];
    for i in 0..n + 1 {
        v[i].sort_unstable();
        for (j, vv) in v[i].iter().enumerate() {
            let mut char_f = to_6digit_chars(i);
            let mut char_b = to_6digit_chars(j + 1);
            char_f.append(&mut char_b);
            ans[vv.1] = char_f.iter().collect::<String>();
        }
    }

    for a in &ans {
        println!("{a}");
    }
}
