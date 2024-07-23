#include <algorithm>
#include <iostream>
#include <vector>

#define ll long long

using namespace std;

int main(void) {
    int n, k;
    cin >> n >> k;
    vector<pair<ll, ll>> abn;
    for (auto i=0; i<n; i++) {
        ll a, b;
        cin >> a >> b;
        abn.push_back(make_pair(a, b));
    }

    // 動けるだけ動いて一気にお金を受け取ろう
    // 累積和と二分探索が浮かんだが, もう少し簡単に線形探索でよい

    sort(abn.begin(), abn.end());
    ll ans = k;
    for (const auto& ab: abn) {
        if (ans < ab.first) {
            // たどりつけない
            break;
        }

        ans += ab.second;
    }

    cout << ans << endl;

    return 0;
}
