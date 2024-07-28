#include <climits>
#include <iostream>

using namespace std;

int main(void) {
  int n, s, m, l;
  cin >> n >> s >> m >> l;

  int ans = INT_MAX;

  for (int i=0; i<100; i++) {
    for (int j=0; j<100; j++) {
      for (int k=0; k<100; k++) {
        int eggs = 6*i + 8*j + 12*k;
        if (eggs >= n) {
          int cost = s*i + m*j + l*k;
          ans = min(ans, cost);
        }
      }
    }
  }

  cout << ans << endl;

  return 0;
}
