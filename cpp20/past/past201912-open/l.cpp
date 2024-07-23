#include <cmath>
#include <iomanip>
#include <iostream>
#include <queue>
#include <vector>

using namespace std;

// https://algo-method.com/descriptions/136
struct UnionFind {
    vector<int> par, rank, siz;

    // 構造体の初期化
    UnionFind(int n) : par(n,-1), rank(n,0), siz(n,1) { }

    // 根を求める
    int root(int x) {
        if (par[x]==-1) return x; // x が根の場合は x を返す
        else return par[x] = root(par[x]); // 経路圧縮
    }

    // x と y が同じグループに属するか (= 根が一致するか)
    bool issame(int x, int y) {
        return root(x)==root(y);
    }

    // x を含むグループと y を含むグループを併合する
    bool unite(int x, int y) {
        int rx = root(x), ry = root(y); // x 側と y 側の根を取得する
        if (rx==ry) return false; // すでに同じグループのときは何もしない
        // union by rank
        if (rank[rx]<rank[ry]) swap(rx, ry); // ry 側の rank が小さくなるようにする
        par[ry] = rx; // ry を rx の子とする
        if (rank[rx]==rank[ry]) rank[rx]++; // rx 側の rank を調整する
        siz[rx] += siz[ry]; // rx 側の siz を調整する
        return true;
    }

    // x を含む根付き木のサイズを求める
    int size(int x) {
        return siz[root(x)];
    }
};

int main(void) {
    using tower = pair<pair<int, int>, int>;
    auto tower_dist2 = [](tower a, tower b) {
        // 誤差避けのため整数で計算する
        auto dist =pow(a.first.first-b.first.first, 2) + pow(a.first.second-b.first.second, 2);
        if (a.second == b.second) {
            return dist;
        } else {
            return 100*dist;
        }
    };

    using edge = pair<int, pair<int, int>>;
    auto cmp = [](const edge& l, const edge& r) {
        return l > r;
    };

    int n, m;
    cin >> n >> m;
    vector<tower> towers_big;
    for(auto i=0; i<n; i++) {
        int x, y, c;
        cin >> x >> y >> c;
        towers_big.push_back(make_pair(make_pair(x, y), c));
    }
    vector<tower> towers_small;
    for(auto i=0; i<m; i++) {
        int x, y, c;
        cin >> x >> y >> c;
        towers_small.push_back(make_pair(make_pair(x, y), c));
    }

    auto ans = 1e12;
    // 連結する小さい塔を全通り試す
    for (auto bit=0; bit<pow(2, m); bit++) {
        vector<bool> use_small(m, false);
        auto b = bit;
        for (auto i=0; i<m; i++) {
            if (b%2 == 1) {
                use_small[i] = true;
            }
            b /= 2;
        }

        // MST はコスト小さい順に拾いたい
        priority_queue<edge, vector<edge>, decltype(cmp)> pq(cmp);
        // 大きい塔同士の接続
        for (auto i=0; i<n; i++) {
            for (auto j=i+1; j<n; j++) {
                pq.push(
                    make_pair(
                        tower_dist2(towers_big[i], towers_big[j]),
                        make_pair(i, j)
                    )
                );
            }
        }

        // 大きい塔と小さい塔の連結
        for (auto i=0; i<n; i++) {
            for (auto j=0; j<m; j++) {
                if (!use_small[j]) {
                    continue;
                }

                pq.push(
                    make_pair(
                        tower_dist2(towers_big[i], towers_small[j]),
                        make_pair(i, j+n)
                    )
                );
            }
        }

        // 小さい塔同士の連結
        for (auto i=0; i<m; i++) {
            if (!use_small[i]) {
                continue;
            }

            for (auto j=i+1; j<m; j++) {
                if (!use_small[j]) {
                    continue;
                }

                pq.push(
                    make_pair(
                        tower_dist2(towers_small[i], towers_small[j]),
                        make_pair(i+n, j+n)
                    )
                );
            }
        }

        UnionFind uf = UnionFind(n + m);
        auto cost_cur = 0.0;
        // すべての辺を見るので完成したかの判定は不要
        while (!pq.empty()) {
            edge e = pq.top();
            pq.pop();

            auto f = e.second.first;
            auto s = e.second.second;
            if (uf.issame(f, s)) {
                continue;
            }

            uf.unite(f, s);
            cost_cur += sqrt(e.first);
        }

        ans = min(ans, cost_cur);
    }

    cout << fixed << setprecision(10) << ans << endl;

    return 0;
}
