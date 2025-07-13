using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main()
    {
        const long UNVISITED = -1;

        var nm = Console.ReadLine().Split();
        var n = int.Parse(nm[0]);
        var m = int.Parse(nm[1]);
        var an = Console.ReadLine().Split().Select(long.Parse).ToArray();

        // dp[i, j, k]: i 日目完了時点で j 日宿題をした場合の楽しさ合計最大値
        //              k=0 なら前日には宿題をしていない
        var dp = new long[n + 1, m + 1, 2];
        for (int i = 0; i <= n; i++)
            for (int j = 0; j <= m; j++)
                for (int k = 0; k < 2; k++)
                    dp[i, j, k] = UNVISITED;
        dp[0, 0, 0] = 0;

        for (int i = 0; i < n; i++)
        {
            for (int j = 0; j <= m; j++)
            {
                // 宿題をしない
                if (dp[i, j, 0] != UNVISITED)
                    dp[i + 1, j, 0] = Math.Max(dp[i + 1, j, 0], dp[i, j, 0] + an[i]);
                if (dp[i, j, 1] != UNVISITED)
                    dp[i + 1, j, 0] = Math.Max(dp[i + 1, j, 0], dp[i, j, 1] + an[i]);

                // 宿題をする
                if (j != m && dp[i, j, 0] != UNVISITED)
                    dp[i + 1, j + 1, 1] = Math.Max(dp[i + 1, j + 1, 1], dp[i, j, 0]);
            }
        }

        long ans = Math.Max(dp[n, m, 0], dp[n, m, 1]);
        Console.WriteLine(ans);
    }
}