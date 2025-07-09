using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main()
    {
        int n = int.Parse(Console.ReadLine());
        var an = Console.ReadLine().Split().Select(long.Parse).ToArray();

        // dp[i][j]: [l, r) の最善値
        var dp = new long[n + 1, n + 1];

        for (int len = 1; len <= n; len++)
        {
            for (int i = 0; i + len <= n; i++)
            {
                int j = i + len;
                if ((n - len) % 2 == 0)
                    // 先手
                    dp[i, j] = Math.Max(dp[i + 1, j] + an[i], dp[i, j - 1] + an[j - 1]);
                else
                    // 後手
                    dp[i, j] = Math.Min(dp[i + 1, j] - an[i], dp[i, j - 1] - an[j - 1]);
            }
        }

        Console.WriteLine(dp[0, n]);
    }
}