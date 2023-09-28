use proconio::input;
use permutohedron::heap_recursive;

fn main() {
    input! {
        n: usize,
        k: u64,
        t: [[u64; n]; n],
    }

    let mut pts = vec![];
    for i in 1..n {
        pts.push(i);
    }
    let mut routes = vec![];
    heap_recursive(&mut pts, |p| {
        routes.push(p.to_vec());
    });
    // println!("{:?}", routes);

    let mut ans = 0u64;
    for r in 0..routes.len() {
        // println!("{:?}", routes[r]);
        let mut cur = 0;
        for i in 0..routes[r].len() {
            if i == 0 {
                cur += t[0][routes[r][i]];
            } else {
                cur += t[routes[r][i - 1]][routes[r][i]];
            }
        }
        cur += t[routes[r][routes[r].len() - 1]][0];
        if cur == k {
            ans += 1;
        }
    }

    println!("{}", ans);
}
