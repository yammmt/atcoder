// 009 - Brute Force 2

#include <iostream>
#include <vector>

using namespace std;

int main(void) {
  int n, s;
  cin >> n >> s;
  vector<int> an;
  for (int i=0; i<n; i++) {
    int a;
    cin >> a;
    an.push_back(a);
  }

  vector<int> dp(s+1, false);
  dp[0] = true;
  for (int i=0; i<n; i++) {
    vector<int> dp_nxt(s+1, false);
    for (int j=0; j<s+1; j++){
      if (!dp[j]) {
        continue;
      }

      dp_nxt[j] = true;
      int j_nxt = j+an[i];
      if (j_nxt <= s) {
        dp_nxt[j_nxt] = true;
      }
    }
    dp = dp_nxt;
  }

  if (dp[s]) {
    cout << "Yes" << endl;
  } else {
    cout << "No" << endl;
  }

  return 0;
}
