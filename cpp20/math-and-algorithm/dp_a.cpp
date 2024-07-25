#include <climits>
#include <iostream>
#include <vector>

using namespace std;

int main(void) {
  int n;
  cin >> n;
  vector<int> hn;
  for (int i=0; i<n; i++) {
    int h;
    cin >> h;
    hn.push_back(h);
  }

  vector<int> dp(n, INT_MAX/2);
  dp[0] = 0;
  for (int i=0; i<n; i++) {
    // 配る DP
    if (i+1 < n) {
      dp[i+1] = min(dp[i+1], dp[i]+abs(hn[i]-hn[i+1]));
    }
    if (i+2 < n) {
      dp[i+2] = min(dp[i+2], dp[i]+abs(hn[i]-hn[i+2]));
    }
  }

  cout << dp[n-1] << endl;

  return 0;
}
