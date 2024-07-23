#include <iostream>
#include <vector>

using namespace std;

int main(void) {
    string s;
    cin >> s;

    auto ans = 0;
    for (auto i=0; i<10000; i++) {
        bool is_candidate = true;
        vector<bool> number_appears(10, false);

        // 個々の数字が含まれているかを判定
        auto ii = i;
        for (auto j=0; j<4; j++) {
            number_appears[ii%10] = true;

            if (s[ii%10] == 'x') {
                is_candidate = false;
                break;
            }
            ii /= 10;
        }

        // 'o' がすべて含まれているかを判定
        for (auto j=0; j<10; j++) {
            if (s[j]=='o' && !number_appears[j]) {
                is_candidate = false;
                break;
            }
        }

        if (is_candidate) {
            ans++;
        }
    }

    cout << ans << endl;

    return 0;
}
