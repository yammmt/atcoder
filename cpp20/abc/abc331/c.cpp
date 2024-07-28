#include <algorithm>
#include <iostream>
#include <vector>

#define LL long long

using namespace std;

int main(void) {
  int n;
  cin >> n;
  vector<LL> an;
  vector<LL> aan;
  for (int i=0; i<n; i++) {
    LL a;
    cin >> a;
    an.push_back(a);
    aan.push_back(a);
  }
  sort(aan.begin(), aan.end());

  vector<LL> a_to_sum(aan[n-1]+1, 0);
  LL cusum = 0;
  for (int i=n-1; i>=0; i--) {
    if ((i<n-1) && (aan[i]!=aan[i+1])) {
      a_to_sum[aan[i]] = cusum;
    }

    cusum += aan[i];
  }

  for (int i=0; i<n; i++) {
    cout << a_to_sum[an[i]];
    if (i==n-1) {
      cout << endl;
    } else {
      cout << " ";
    }
  }

  return 0;
}
