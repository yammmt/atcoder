#include <iostream>
#include <vector>

using namespace std;

int main(void) {
  int n;
  cin >> n;
  vector<int> tn;
  for (int i=0; i<n; i++) {
    int t;
    cin >> t;
    tn.push_back(t);
  }

  int tsum = 0;
  for (const auto& t: tn) {
    tsum += t;
  }

  vector<int> dp(tsum+1, false);
  dp[0] = true;
  for (int i=0; i<n; i++) {
    vector<int> dp_nxt(tsum+1, false);
    for (int j=0; j<=tsum; j++){
      if (!dp[j]) {
        continue;
      }

      dp_nxt[j] = true;
      dp_nxt[j+tn[i]] = true;
    }
    dp = dp_nxt;
  }

  for (int i=(tsum+1)/2; i<tsum+1; i++){
    if (dp[i]) {
      cout << i << endl;
      return 0;
    }
  }

  return 0;
}
