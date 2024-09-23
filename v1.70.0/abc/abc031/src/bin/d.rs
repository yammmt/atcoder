use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        k: usize,
        n: usize,
        vwn_before: [(usize, Chars); n],
    }

    // 一意に復号可能とは限らないところが厄介
    // アルファベット 27 字の組み方を考えようにも 27! 通りに区切りまで考えるので TLE する
    // それでも部分点は通りそうだが

    // - v_i は "1,000,000,000" みたいな文字列として見做せる
    // - w_i の長さは最大でも 10x3 = 30 文字
    // 文字列の長さを決め打って全探索

    let mut vwn = vec![];
    for (mut v, w) in vwn_before {
        let mut vv = vec![];
        while v > 0 {
            vv.push(v % 10);
            v /= 10;
        }
        vv.reverse();
        vwn.push((vv, w));
    }

    for mut b in 0..3usize.pow(k as u32) {
        // println!("{b}");
        let mut num_len = vec![0];
        for _ in 0..k {
            num_len.push(b % 3 + 1);
            b /= 3;
        }
        // println!("  num_len: {:?}", num_len);
        // ここまではあっている

        let mut num2chars: Vec<Option<Vec<char>>> = vec![None; k + 1];
        let mut could_ans = true;
        'vwn_loop: for (v, w) in &vwn {
            let mut w_i = 0;

            let mut nv_len = 0;
            for &vv in v {
                nv_len += num_len[vv];
            }
            if nv_len != w.len() {
                // no answer
                could_ans = false;
                break;
            }

            for &vv in v {
                if let Some(vc) = &num2chars[vv] {
                    for j in 0..num_len[vv] {
                        if vc[j] != w[w_i + j] {
                            could_ans = false;
                            break 'vwn_loop;
                        }
                    }
                } else {
                    let mut vc = vec![];
                    for i in 0..num_len[vv] {
                        vc.push(w[w_i + i]);
                    }
                    num2chars[vv] = Some(vc);
                }
                w_i += num_len[vv];
            }
        }

        if could_ans {
            // println!("{:?}", num2chars);
            for co in num2chars.iter().skip(1) {
                if let Some(c) = co {
                    println!("{}", c.iter().collect::<String>());
                }
            }

            return;
        }
    }
    // 結果からバグ有無を判定するため
    unreachable!();
}
