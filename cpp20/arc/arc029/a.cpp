#include <bitset>
#include <iostream>
#include <vector>

using namespace std;

int main(void) {
  int n;
  cin >> n;
  vector<int> tn;
  for (auto i=0; i<n; i++) {
    int a;
    cin >> a;
    tn.push_back(a);
  }
  // bitset を用いる都合, tn の長さを固定値にしたい
  while (tn.size() < 4) {
    tn.push_back(0);
  }

  auto ans = 1'000'000'000;
  for (auto bit=0; bit<(1<<n); bit++) {
    bitset<4> bs(bit);
    auto meat1 = 0;
    auto meat2 = 0;
    for (auto i=0; i<n; i++) {
      if (bs.test(i) == 1) {
        meat1 += tn[i];
      } else {
        meat2 += tn[i];
      }
    }

    ans = min(ans, max(meat1, meat2));
  }

  cout << ans << endl;

  return 0;
}
