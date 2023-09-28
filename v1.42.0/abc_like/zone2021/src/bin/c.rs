// 問題文がややこしかったが読めても辛い

use proconio::input;

fn main() {
    input! {
        n: usize,
        abcden: [(i64, i64, i64, i64, i64); n],
    }

    // 3000C3 は TLE
    let mut pass = 0;
    let mut fail = 1_000_000_001;
    while fail - pass > 1 {
        let mid = (pass + fail) / 2;
        // println!("{} {} => {}", pass, fail, mid);

        let mut candidates = vec![];
        for abcde in &abcden {
            candidates.push((
                if abcde.0 >= mid {
                    1
                } else {
                    0
                },
                if abcde.1 >= mid {
                    1
                } else {
                    0
                },
                if abcde.2 >= mid {
                    1
                } else {
                    0
                },
                if abcde.3 >= mid {
                    1
                } else {
                    0
                },
                if abcde.4 >= mid {
                    1
                } else {
                    0
                },
            ));
        }
        candidates.sort();
        candidates.dedup();

        let clen = candidates.len();
        let mut cur_pass = false;

        if clen < 3 {
            let mut ability = (0, 0, 0, 0, 0);
            for c in &candidates {
                ability.0 |= c.0;
                ability.1 |= c.1;
                ability.2 |= c.2;
                ability.3 |= c.3;
                ability.4 |= c.4;
            }
            if ability.0 ==1 && ability.1 == 1 && ability.2 == 1
                && ability.3 == 1 && ability.4 == 1
            {
                cur_pass = true;
            }
        } else {
            'i_loop: for i in 0..clen {
                for j in i + 1..clen {
                    for k in j + 1..clen {
                        let ability = (
                            candidates[i].0 | candidates[j].0 | candidates[k].0,
                            candidates[i].1 | candidates[j].1 | candidates[k].1,
                            candidates[i].2 | candidates[j].2 | candidates[k].2,
                            candidates[i].3 | candidates[j].3 | candidates[k].3,
                            candidates[i].4 | candidates[j].4 | candidates[k].4,
                        );
                        if ability.0 ==1 && ability.1 == 1 && ability.2 == 1
                            && ability.3 == 1 && ability.4 == 1
                        {
                            cur_pass = true;
                            break 'i_loop;
                        }
                    }
                }
            }
        }

        if cur_pass {
            pass = mid;
        } else {
            fail = mid;
        }
    }

    println!("{}", pass);
}
