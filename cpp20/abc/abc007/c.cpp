#include <iostream>
#include <queue>

#define DUMMY 1000000

using namespace std;

int main(void) {
    int r, c;
    cin >> r >> c;
    pair<int, int> p_start, p_goal;
    int x, y;
    cin >> x >> y;
    p_start = make_pair(x-1, y-1);
    cin >> x >> y;
    p_goal = make_pair(x-1, y-1);
    vector<string> crc;
    for (auto i=0; i<r; i++) {
        string s;
        cin >> s;
        crc.push_back(s);
    }

    vector<vector<int>> scores(r, vector<int>(c, DUMMY));
    queue<pair<int, int>> q;
    q.push(p_start);
    scores[p_start.first][p_start.second] = 0;
    while (q.size() != 0) {
        auto p = q.front();
        q.pop();

        for (auto di=-1; di<=1; di++) {
            for (auto dj=-1; dj<=1; dj++) {
                if (abs(di + dj) != 1) {
                    continue;
                }

                auto i = p.first+di;
                auto j = p.second+dj;
                if (
                    (i < 0) || (j < 0) || (i >= r) || (j >= c)
                    || (scores[i][j] != DUMMY)
                    || (crc[i][j] == '#')
                ) {
                    continue;
                }

                q.push(make_pair(i, j));
                scores[i][j] = scores[p.first][p.second]+1;
            }
        }
    }

    cout << scores[p_goal.first][p_goal.second] << endl;

    return 0;
}
