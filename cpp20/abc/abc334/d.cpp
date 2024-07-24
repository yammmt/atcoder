#include <algorithm>
#include <iostream>
#include <vector>

#define LL long long

using namespace std;

int main(void) {
    int n, q;
    cin >> n >> q;
    vector<LL> rn;
    for (auto i=0; i<n; i++) {
        LL r;
        cin >> r;
        rn.push_back(r);
    }
    vector<LL> qq;
    for (auto i=0; i<q; i++) {
        LL a;
        cin >> a;
        qq.push_back(a);
    }

    sort(rn.begin(), rn.end());
    vector<LL> reindeer_sum;
    reindeer_sum.push_back(0);
    LL cusum = 0;
    for (const auto& r: rn) {
        cusum += r;
        reindeer_sum.push_back(cusum);
    }

    for (const auto& a: qq) {
        // cout << "a: " << a << endl;
        auto i = upper_bound(reindeer_sum.begin(), reindeer_sum.end(), a);
        cout << i-reindeer_sum.begin()-1 << endl;
    }

    return 0;
}
