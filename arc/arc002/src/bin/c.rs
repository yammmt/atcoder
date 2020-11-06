use proconio::input;

// 18min

fn main() {
    input! {
        n: usize,
        s: String,
    }

    let vc = s.chars().collect::<Vec<char>>();
    let cmds = ["AA", "AB", "AX", "AY", "BA", "BB", "BX", "BY", "XA", "XB", "XX", "XY", "YA", "YB", "YX", "YY"];
    let mut ans = std::u32::MAX;
    for i in 0..cmds.len() {
        for j in i + 1..cmds.len() {
            let mut cmd_num = 0;
            let mut just_before_cmd = false;
            for c in 1..n {
                let ss = vec![vc[c - 1], vc[c]].iter().collect::<String>();
                if !just_before_cmd {
                    if ss == cmds[i] || ss == cmds[j] {
                        cmd_num += 1;
                        just_before_cmd = true;
                    }
                } else {
                    just_before_cmd = false;
                }
            }
            ans = ans.min(n as u32 - cmd_num);
        }
    }
    println!("{}", ans);
}
