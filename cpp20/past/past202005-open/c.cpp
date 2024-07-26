#include <iostream>

using namespace std;

int main(void) {
  int a, r, n;
  cin >> a >> r >> n;

  int ans = a;
  for (int i=0; i<n-1; i++) {
    if (ans > 1'000'000'000/r) {
      cout << "large" << endl;
      return 0;
    }

    ans *= r;
  }

  cout << ans << endl;

  return 0;
}
