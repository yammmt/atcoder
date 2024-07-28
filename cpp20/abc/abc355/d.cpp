// 全く同じ
// https://app.codility.com/programmers/lessons/6-sorting/number_of_disc_intersections/

// 解説放送の l, r 独立にソートしてよいという考えは
// l < r が成り立つため, その地点では閉じてしまっている区間を数えるもの

#include <algorithm>
#include <iostream>
#include <tuple>
#include <vector>

#define LL long long

using namespace std;

int main(void) {
  int n;
  cin >> n;
  vector<tuple<int, int>> vevents;
  for (int i=0; i<n; i++) {
    int l, r;
    cin >> l >> r;
    vevents.push_back(make_tuple(l, 1));
    vevents.push_back(make_tuple(r+1,-1));
  }
  sort(vevents.begin(), vevents.end());

  LL ans = 0;
  LL interval_num = 0;
  for (const auto& e: vevents) {
    int pos, diff;
    tie(pos, diff) = e;
    if (diff == 1) {
      ans += interval_num;
      interval_num++;
    } else {
      interval_num--;
    }
  }

  cout << ans << endl;

  return 0;
}
