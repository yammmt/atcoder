#include <iostream>
#include <vector>

#define LL long long

using namespace std;

int main(void) {
  // 置き換えなくてよい場合を 1 として数えるらしい

  int n;
  cin >> n;
  vector<int> an;
  for (int i=0; i<n; i++) {
    int a;
    cin >> a;
    a--;
    an.push_back(a);
  }

  for (int i=0; i<n; i++) {
    int ans = 1;
    int x = an[i];
    while (x != i) {
      x = an[x];
      ans++;
    }

    cout << ans;
    if (i == n-1) {
      cout << endl;
    } else {
      cout << " ";
    }
  }

  return 0;
}
