#include <algorithm>
#include <iostream>
#include <vector>

#define LL long long

using namespace std;

int main(void) {
  int n, m;
  cin >> n >> m;
  vector<int> an;
  for (int i=0; i<n; i++) {
    LL a;
    cin >> a;
    an.push_back(a);
  }
  vector<int> bm;
  for (int i=0; i<m; i++) {
    LL b;
    cin >> b;
    bm.push_back(b);
  }

  // sort して尺取り

  sort(an.begin(), an.end());
  sort(bm.begin(), bm.end());

  LL ans = 0;
  int i_a = 0;
  // 条件を満たす最小の数を貪欲に買っていくとよい
  for (const auto& b: bm) {
    bool could_buy = false;
    while (i_a < n) {
      if (b <= an[i_a]) {
        could_buy = true;
        ans += an[i_a];
        i_a++;
        break;
      }

      i_a++;
    }

    if (!could_buy) {
      cout << -1 << endl;
      return 0;
    }
  }

  cout << ans << endl;

  return 0;
}
