#include <iostream>
#include <queue>
#include <vector>

using namespace std;

int main(void) {
  int n, m;
  cin >> n >> m;
  vector<vector<int>> edges(n);
  for (int i=0; i<m; i++) {
    int a, b;
    cin >> a >> b;
    a--;
    b--;
    edges[a].push_back(b);
  }

  // 個々の点から BFS するだけでも間に合いそう

  int ans = 0;
  for (int i=0; i<n; i++) {
    vector<bool> visited(n, false);
    queue<int> q;
    q.push(i);
    // cout << "i: " << i << endl;
    while (!q.empty()) {
      int v_cur = q.front();
      q.pop();
      if (visited[v_cur]) {
        continue;
      }

      visited[v_cur] = true;
      for (const auto& e: edges[v_cur]) {
        q.push(e);
      }
    }

    for (int j=0; j<n; j++) {
      if (visited[j]) {
        ans++;
      }
    }
  }

  cout << ans << endl;

  return 0;
}
