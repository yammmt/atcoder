use itertools::Itertools;
use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut an: [usize; n],
    }

    if n == 1 {
        // インデックス管理恐いので例外として
        println!("{}", an[0]);
        return;
    }

    // 質問回答よりシェイクは一度きり
    // 最短の棒は割れた後の最長の棒と組み合わせねばならない
    // すると L は高々 2 つしかないような？雑に書いても通る？
    // 嫌な入力は [5, 5, 10, ..., 10] のようなものか
    // ちょっと遅いが, 配列複製してインデックス合わせたほうが実装は楽かも

    an.sort_unstable();

    let mut ans = vec![];

    // 最長の棒が割られていない場合
    let l_candidate = *an.last().unwrap();
    let mut last_i = n - 2;
    while last_i > 0 && an[last_i] == l_candidate {
        last_i -= 1;
    }
    if last_i == 0 && an[0] == l_candidate {
        // 全部割られていない
        ans.push(l_candidate);
    } else if (last_i + 1) % 2 == 0 {
        // 奇数個だとペアが作れないので先に弾く
        let mut passes = true;
        for i in 0..last_i.div_ceil(2) {
            let j = last_i - i;
            if an[i] + an[j] != l_candidate {
                passes = false;
                break;
            }
        }
        if passes {
            ans.push(l_candidate);
        }
    }

    // 最長の棒が割られていた場合
    if n % 2 == 0 {
        let l_candidate = *an.first().unwrap() + *an.last().unwrap();
        let mut passes = true;
        for i in 0..n / 2 {
            let j = n - i - 1;
            if an[i] + an[j] != l_candidate {
                passes = false;
                break;
            }
        }
        if passes {
            ans.push(l_candidate);
        }
    }

    println!("{}", ans.iter().join(" "));
}
