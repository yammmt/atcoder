use proconio::input;

fn main() {
    input! {
        n: u32,
        an: [u64; 2u64.pow(n) as usize],
    }

    let mut ans = vec![0; 2u64.pow(n) as usize];
    let mut entries = vec![];
    for (i, a) in an.iter().enumerate() {
        entries.push((i, *a));
    }

    let mut finish = false;
    let mut score = 1;
    let mut next_entries = vec![];
    while !finish {
        for i in 0..entries.len() {
            if i % 2 == 1 {
                continue;
            }

            if entries[i].1 < entries[i + 1].1 {
                ans[entries[i].0] = score;
                next_entries.push(entries[i + 1]);
            } else {
                ans[entries[i + 1].0] = score;
                next_entries.push(entries[i]);
            }
        }
        score += 1;
        if next_entries.len() == 2 {
            ans[next_entries[0].0] = score;
            ans[next_entries[1].0] = score;
            finish = true;
        } else if next_entries.len() == 1 {
            ans[next_entries[0].0] = score - 1;
            finish = true;
        }
        entries = next_entries.clone();
        next_entries.clear();
    }

    ans.iter().for_each(|a| println!("{}", a));
}
