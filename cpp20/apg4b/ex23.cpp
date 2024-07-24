#include <algorithm>
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
        an.push_back(a);
    }

    n++;
    an.push_back(2'000'000'000);

    sort(an.begin(), an.end());
    auto ans = 0;
    // 登場しない数
    auto num_cur = 2'000'000'000;
    auto cnt_max = 0;
    auto cnt_cur = 0;
    for (int i=0; i<n; i++) {
        if (an[i] != num_cur) {
            if (cnt_max < cnt_cur) {
                cnt_max = cnt_cur;
                ans = num_cur;
            }
            cnt_cur = 0;
        }

        cnt_cur++;
        num_cur = an[i];
    }

    cout << ans << " " << cnt_max << endl;

    return 0;
}
