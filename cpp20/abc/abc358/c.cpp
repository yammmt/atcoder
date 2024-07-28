#include <cmath>
#include <iostream>
#include <vector>

using namespace std;

int main(void) {
  int n, m;
  cin >> n >> m;
  vector<string> sn;
  for (int i=0; i<n; i++) {
    string s;
    cin >> s;
    sn.push_back(s);
  }

  int ans = n+1;
  for (int bit=0; bit<pow(2, n); bit++) {
    vector<bool> could_buy(m, false);
    int b = bit;
    int score_cur = 0;
    for (int i=0; i<n; i++) {
      if (b%2 == 1) {
        for (int j=0; j<m; j++) {
          if (sn[i][j] == 'o') {
            could_buy[j] = true;
          }
        }
        score_cur++;
      }

      b /= 2;
    }

    bool completed = true;
    for (auto const& c: could_buy) {
      if (!c) {
        completed = false;
        break;
      }
    }
    if (completed) {
      // cout << "  score: " << score_cur << endl;
      ans = min(ans, score_cur);
    }
  }

  cout << ans << endl;

  return 0;
}
