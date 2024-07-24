// testcases 間違ってない？ sample すら形式が違う
// 答えが複数個ある影響？

#include <iostream>
#include <vector>
#include <queue>

#define LL long long

using namespace std;

struct Edge {
    LL cost;
    int vertex;
    int id;
};

int main(void) {
    int n, m;
    cin >> n >> m;
    // (費用, 移動先, 枝番号)
    vector<vector<Edge>> edges(n);
    for (auto i=0; i<m; i++) {
        int a, b;
        LL c;
        cin >> a >> b >> c;
        a--;
        b--;
        edges[a].push_back(Edge{c, b, i+1});
        edges[b].push_back(Edge{c, a, i+1});
    }

    vector<bool> is_arrived(n, false);
    vector<bool> use_edge(m+1, false);
    auto use_edge_num = 0;

    auto cmp = [](const Edge& l, const Edge& r) {
        return l.cost > r.cost;
    };
    // (合計費用, 到着先, 枝番号)
    priority_queue<Edge, vector<Edge>, decltype(cmp)> pq(cmp);
    pq.push(Edge{0, 0, 0});

    while (!pq.empty()) {
        auto cur = pq.top();
        pq.pop();

        if (is_arrived[cur.vertex]) {
            continue;
        }

        // 到着の反映
        // cout << cost_cur << "," << v_cur << "," << edge_cur << endl;
        is_arrived[cur.vertex] = true;
        use_edge[cur.id] = true;
        use_edge_num++;
        if (use_edge_num == n) {
            // ==n: 始点にダミーの辺を用意しているので
            break;
        }

        // 到着した頂点からの移動分
        for (const auto& nxt: edges[cur.vertex]) {
            pq.push(Edge{cur.cost+nxt.cost, nxt.vertex, nxt.id});
        }
    }


    auto ans_num = 0;
    for (auto i=1; i<m+1; i++) {
        if (use_edge[i]) {
            cout << i;
            ans_num++;
            if (ans_num == n-1) {
                cout << endl;
            } else {
                cout << " ";
            }
        }
    }

    return 0;
}
