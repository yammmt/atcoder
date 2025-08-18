using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    const long unvisited = 1 << 60;

    static long[] minpath(int vbegin, List<int> vend, List<List<int>> edges)
    {
        var n = edges.Count;
        var scores = new long[n];
        Array.Fill(scores, unvisited);

        var que = new Queue<(int, long)>();
        que.Enqueue((vbegin, 0));
        while (que.TryDequeue(out (int v, long score) cur))
        {
            if (scores[cur.v] != unvisited)
                continue;

            scores[cur.v] = cur.score;
            foreach (var vnext in edges[cur.v])
                que.Enqueue((vnext, cur.score + 1));
        }

        var nret = vend.Count;
        var ret = new long[nret];
        for (int i = 0; i < nret; i++)
            ret[i] = scores[vend[i]];

        return ret;
    }

    static void Main()
    {
        // 回る順を 16! 通りで計算すると TLE
        // 始点は高々 16 点であるので, 各点への最短経路を Dijkstra する
        // 求まった最短経路を使って bitDP で周り順を決める
        // 各回の移動は独立であるので扱い易い…が, 重実装
        // ...辺の重み固定だし Dijkstra する必要ないな？通るはずだが

        var nm = Console.ReadLine().Split(' ');
        var n = int.Parse(nm[0]);
        var m = int.Parse(nm[1]);
        var edges = new List<List<int>>();
        for (int i = 0; i < n; i++)
            edges.Add(new List<int>());
        for (int i = 0; i < m; i++)
        {
            var uv = Console.ReadLine().Split(' ');
            var u = int.Parse(uv[0]) - 1;
            var v = int.Parse(uv[1]) - 1;
            edges[u].Add(v);
            edges[v].Add(u);
        }
        var s = int.Parse(Console.ReadLine()) - 1;
        var k = int.Parse(Console.ReadLine());
        // tk は sorted とは限らないが, 無視してもよいはず
        var tk = Console.ReadLine().Split(' ').Select(a => int.Parse(a) - 1).ToList();

        // 訪ねる街間の最短経路パート
        var edgesTk = new long[k, k];
        for (int i = 0; i < k; i++)
        {
            var dist = minpath(tk[i], tk, edges);
            for (int j = 0; j < k; j++)
                edgesTk[i, j] = dist[j];
        }
        // s から出発する分のコストを初期値としてもつ
        var fromS = minpath(s, tk, edges);

        // bitDP パート
        var kbit = 1 << k;
        var dp = new long[kbit, k];
        for (int i = 0; i < kbit; i++)
            for (int j = 0; j < k; j++)
                dp[i, j] = unvisited;
        for (int i = 0; i < k; i++)
            dp[1 << i, i] = fromS[i];

        for (int i = 0; i < kbit; i++)
        {
            for (int j = 0; j < k; j++)
            {
                if (dp[i, j] == unvisited)
                    continue;

                for (int kk = 0; kk < k; kk++)
                {
                    var iNext = i | (1 << kk);
                    dp[iNext, kk] = Math.Min(dp[iNext, kk], dp[i, j] + edgesTk[j, kk]);
                }
            }
        }

        var ans = unvisited;
        for (int i = 0; i < k; i++)
            ans = Math.Min(ans, dp[kbit - 1, i]);
        Console.WriteLine(ans);
    }
}
