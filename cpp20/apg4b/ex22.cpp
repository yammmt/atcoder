#include <algorithm>
#include <iostream>
#include <tuple>
#include <vector>

using namespace std;

int main(void) {
    int n;
    cin >> n;
    vector<tuple<int, int>> ban;
    for (auto i=0; i<n; i++) {
        int a, b;
        cin >> a >> b;
        ban.push_back(tuple(b, a));
    }
    sort(ban.begin(), ban.end());

    for (const auto& ba: ban) {
        int a, b;
        tie(b, a) = ba;
        cout << a << " " << b << endl;
    }

    return 0;
}
