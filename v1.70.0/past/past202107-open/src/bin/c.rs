use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars,
        mut l: usize,
        mut r: usize,
    }

    // TODO: Chars で半角区切りどうだったっけ

    if s[0] == '0' && s.len() > 1 {
        println!("No");
        return;
    }

    // 0 を足しやすく
    // 12345 => 54321
    s.reverse();

    let mut ls = vec![];
    while l > 0 {
        ls.push(char::from_digit((l % 10) as u32, 10).unwrap());
        l /= 10;
    }

    let mut rs = vec![];
    while r > 0 {
        rs.push(char::from_digit((r % 10) as u32, 10).unwrap());
        r /= 10;
    }

    let max_digit = s.len().max(ls.len().max(rs.len()));
    while s.len() < max_digit {
        s.push('0');
    }
    while ls.len() < max_digit {
        ls.push('0');
    }
    while rs.len() < max_digit {
        rs.push('0');
    }

    s.reverse();
    ls.reverse();
    rs.reverse();

    println!("{}", if ls <= s && s <= rs { "Yes" } else { "No" });
}
