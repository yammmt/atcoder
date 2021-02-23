use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;

fn main() {
    input! {
        s: Chars,
    }

    let mut ans = HashSet::new();
    ans.insert(".".to_string());
    if s.len() > 1 {
        ans.insert(['.', '.'].iter().collect::<String>());
    }
    if s.len() > 2 {
        ans.insert(['.', '.', '.'].iter().collect::<String>());
    }

    for i in 0..s.len() {

        // 1
        ans.insert(s[i].to_string());
        // 2
        if i + 1 >= s.len() {
            continue;
        }

        ans.insert(vec![s[i], s[i + 1]].iter().collect::<String>());
        ans.insert(vec![s[i], '.'].iter().collect::<String>());
        ans.insert(vec!['.', s[i + 1]].iter().collect::<String>());

        // 3
        if i + 2 >= s.len() {
            continue;
        }

        ans.insert(vec![s[i], s[i + 1], s[i + 2]].iter().collect::<String>());
        ans.insert(vec![s[i], s[i + 1], '.'].iter().collect::<String>());
        ans.insert(vec![s[i], '.', s[i + 2]].iter().collect::<String>());
        ans.insert(vec!['.', s[i + 1], s[i + 2]].iter().collect::<String>());
        ans.insert(vec![s[i], '.', '.'].iter().collect::<String>());
        ans.insert(vec!['.', '.', s[i + 2]].iter().collect::<String>());
        ans.insert(vec!['.', s[i + 1], '.'].iter().collect::<String>());
    }

    println!("{}", ans.len());
}
