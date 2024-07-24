#include <iostream>
#include <vector>
#include <queue>

#define LL long long

using namespace std;

int main(void) {
    int n;
    cin >> n;
    vector<int> sn, tn;
    for (auto i=0; i<n; i++) {
        int s;
        cin >> s;
        sn.push_back(s);
    }
    for (auto i=0; i<n; i++) {
        int t;
        cin >> t;
        tn.push_back(t);
    }

    using edge = pair<int, int>;
    auto cmp = [](const edge& l, const edge& r) {
        return l > r;
    };

    // 時刻 0 に受け取ることはないので
    vector<LL> ans(n, 0);
    auto completed_num = 0;
    priority_queue<edge, vector<edge>, decltype(cmp)> pq(cmp);
    for (auto i=0; i<n; i++) {
        pq.push(make_pair(tn[i], i));
    }

    while (pq.size() > 0) {
        auto e = pq.top();
        pq.pop();

        if (ans[e.second] != 0) {
            continue;
        }

        ans[e.second] = e.first;
        completed_num++;
        if (completed_num == n) {
            // answer
            break;
        }

        pq.push(make_pair(e.first+sn[e.second], (e.second+1)%n));
    }

    for (const auto& a: ans) {
        cout << a << endl;
    }

    return 0;
}
