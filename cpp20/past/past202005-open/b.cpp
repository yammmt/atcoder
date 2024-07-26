#include <iostream>
#include <vector>

using namespace std;

int main(void) {
  int n, m, q;
  cin >> n >> m >> q;

  vector<vector<int>> q_to_player(m);
  vector<vector<int>> player_to_q(n);
  for (int i=0; i<q; i++) {
    int a;
    cin >> a;
    if (a == 1) {
      // 出力
      int nn;
      cin >> nn;
      nn--;

      int score = 0;
      for (const auto& qq: player_to_q[nn]) {
        score += n-(int)(q_to_player[qq].size());
      }
      cout << score << endl;
    } else {
      // 解いた
      int nn, mm;
      cin >> nn >> mm;
      nn--;
      mm--;

      q_to_player[mm].push_back(nn);
      player_to_q[nn].push_back(mm);
    }
  }

  return 0;
}
