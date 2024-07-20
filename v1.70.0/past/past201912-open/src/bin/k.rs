use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

const DUMMY: usize = usize::MAX / 2;

fn dfs(
    v: usize,
    edges: &Vec<Vec<usize>>,
    mut order_pre: &mut Vec<usize>,
    mut order_post: &mut Vec<usize>,
    mut cur_pre: &mut usize,
    mut cur_post: &mut usize,
) {
    if order_pre[v] == DUMMY {
        order_pre[v] = *cur_pre;
        *cur_pre += 1;
        for &v_nxt in &edges[v] {
            // DAG だから重複しない, でよいはず
            dfs(
                v_nxt,
                &edges,
                &mut order_pre,
                &mut order_post,
                &mut cur_pre,
                &mut cur_post,
            );
        }
    }

    order_post[v] = *cur_post;
    *cur_post += 1;
}

#[fastout]
fn main() {
    input! {
        n: usize,
        pn: [isize; n],
        q: usize,
        abq: [(Usize1, Usize1); q],
    }

    // 共通祖先を求めてやればよい
    // j が i の部分木である
    //   => DFS で往路到達順が j > i かつ復路到達順が i > j

    // 上司から部下を辿るため
    let mut edges = vec![vec![]; n];
    for (i, &p) in pn.iter().enumerate() {
        if p == -1 {
            continue;
        }

        edges[p as usize - 1].push(i);
    }
    // println!("edges: {:?}", edges);

    let mut president = 0;
    for (i, &p) in pn.iter().enumerate() {
        if p == -1 {
            president = i;
            break;
        }
    }

    let mut order_pre = vec![DUMMY; n];
    let mut order_post = vec![DUMMY; n];
    let mut cur_pre = 0;
    let mut cur_post = 0;
    dfs(
        president,
        &edges,
        &mut order_pre,
        &mut order_post,
        &mut cur_pre,
        &mut cur_post,
    );
    // println!("{:?}", order_pre);
    // println!("{:?}", order_post);

    for (a, b) in abq {
        // a は b の部下？
        println!(
            "{}",
            if order_pre[b] < order_pre[a] && order_post[a] < order_post[b] {
                "Yes"
            } else {
                "No"
            }
        );
    }
}
