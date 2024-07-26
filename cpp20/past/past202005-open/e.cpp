#include <iostream>
#include <vector>

using namespace std;

int main(void) {
  int n, m, q;
  cin >> n >> m >> q;
  vector<vector<int>> edges(n);
  for (int i=0; i<m; i++) {
    int a, b;
    cin >> a >> b;
    a--;
    b--;
    edges[a].push_back(b);
    edges[b].push_back(a);
  }
  vector<int> colors;
  for (int i=0; i<n; i++) {
    int c;
    cin >> c;
    colors.push_back(c);
  }

  for (int i=0; i<q; i++) {
    int a;
    cin >> a;
    if (a == 1) {
      int x;
      cin >> x;
      x--;
      cout << colors[x] << endl;
      for (const auto& v: edges[x]) {
        colors[v] = colors[x];
      }
    } else {
      int x, y;
      cin >> x >> y;
      x--;
      cout << colors[x] << endl;
      colors[x] = y;
    }
  }


  return 0;
}
