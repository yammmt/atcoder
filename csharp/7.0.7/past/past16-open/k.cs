using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main()
    {
        (int, int)[] dir = { (-1, 0), (1, 0), (0, -1), (0, 1) };

        var n = int.Parse(Console.ReadLine());
        var sn = new char[n][];
        for (int i = 0; i < n; i++)
        {
            sn[i] = Console.ReadLine().ToCharArray();
        }

        // 実行時間 8 ms
        // 時間長いし k が大きくなるとほとんど動けなくなるので, 愚直を通す…
        // と考えども, 通過フラグ配列 [1500][1500] を 1500 回も宣言するとだめ
        // k がいくつのときに通過できたかを管理して, 配列の再宣言を回避すればよい？

        var startX = 0;
        var startY = 0;
        for (int i = 0; i < n; i++)
        {
            for (int j = 0; j < n; j++)
            {
                if (sn[i][j] == 'S')
                {
                    startX = i;
                    startY = j;
                }
            }
        }

        // そのマスから何マス進めるか, を事前に計算しておく
        // 愚直に全始点から見ると, この時点で O(n^3) となり TLE
        // x マスを起点に枝刈り付けて見れば O(n^2+n^2) のはず, 4 方向に舐めるだけ
        // という考えは二次元累積和を縦横で考えておけば良い
        var holesH = new int[n, n];
        for (int i = 0; i < n; i++)
        {
            var holes = 0;
            for (int j = 0; j < n; j++)
            {
                if (sn[i][j] == 'X')
                {
                    holes++;
                }
                holesH[i, j] = holes;
            }
        }
        var holesV = new int[n, n];
        for (int i = 0; i < n; i++)
        {
            var holes = 0;
            for (int j = 0; j < n; j++)
            {
                if (sn[j][i] == 'X')
                {
                    holes++;
                }
                holesV[j, i] = holes;
            }
        }

        var canStayK = new int[n, n];
        var costs = new int[n, n];
        for (int i = 1; i < n; i++)
        {
            var canGoal = false;
            // ((x, y), cost)
            var que = new Queue<((int, int), int)>();
            que.Enqueue(((startX, startY), 0));
            while (que.TryDequeue(out var e))
            {
                var ((x, y), c) = e;
                if (canStayK[x, y] == i)
                {
                    continue;
                }

                if (sn[x][y] == 'G')
                {
                    Console.WriteLine(c);
                    canGoal = true;
                    break;
                }

                canStayK[x, y] = i;
                costs[x, y] = c;
                foreach (var d in dir)
                {
                    var xx = x + d.Item1 * i;
                    var yy = y + d.Item2 * i;

                    if (xx < 0 || xx >= n || yy < 0 || yy >= n
                        || (x == xx && holesH[x, y] != holesH[xx, yy])
                        || (y == yy && holesV[x, y] != holesV[xx, yy])
                        || sn[xx][yy] == 'X'
                    )
                    {
                        continue;
                    }

                    // 探索済みはここで枝刈りした方が高速ではあるはず
                    que.Enqueue(((xx, yy), c + 1));
                }
            }

            if (!canGoal)
            {
                Console.WriteLine("-1");
            }
        }
    }
}