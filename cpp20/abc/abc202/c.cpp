#include <iostream>
#include <vector>

using namespace std;

int main(void) {
    int n;
    cin >> n;
    vector<int> an;
    for (auto i=0; i<n; i++) {
        int a;
        cin >> a;
        an.push_back(a-1);
    }
    vector<int> bn;
    for (auto i=0; i<n; i++) {
        int a;
        cin >> a;
        bn.push_back(a-1);
    }
    vector<int> cn;
    for (auto i=0; i<n; i++) {
        int a;
        cin >> a;
        cn.push_back(a-1);
    }

    // B_C_j で作れる数を数え上げておく
    vector<int> cnt(100'001, 0);
    for (const auto& c: cn) {
        auto b = bn[c];
        cnt[b]++;
    }

    long long ans = 0;
    for (const auto& a: an) {
        ans += cnt[a];
    }

    cout << ans << endl;

    return 0;
}
