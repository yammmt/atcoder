using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main(string[] args)
    {
        const long dummy = 1_000_000_000L * 3100L;
        var n = int.Parse(Console.ReadLine());
        var s = Console.ReadLine().ToCharArray();
        var cn = Console.ReadLine().Split(' ').Select(long.Parse).ToArray();
        var dn = Console.ReadLine().Split(' ').Select(long.Parse).ToArray();

        // dp[i][j]: (1-origin) i 文字目まで見て '(' の数が j 個多い場合の最小コスト
        var dp = new long[n + 1][];
        for (int i = 0; i < n + 1; i++)
            dp[i] = Enumerable.Repeat(dummy, n + 1).ToArray();
        dp[0][0] = 0;

        for (int i = 0; i < n; i++)
            for (int j = 0; j < n; j++)
            {
                if (s[i] == '(')
                {
                    // なにもしない
                    dp[i + 1][j + 1] = Math.Min(dp[i + 1][j + 1], dp[i][j]);
                    // 置き換え
                    if (j > 0)
                        dp[i + 1][j - 1] = Math.Min(dp[i + 1][j - 1], dp[i][j] + cn[i]);
                    // 削除
                    dp[i + 1][j] = Math.Min(dp[i + 1][j], dp[i][j] + dn[i]);
                }
                else
                {
                    // なにもしない
                    if (j > 0)
                        dp[i + 1][j - 1] = Math.Min(dp[i + 1][j - 1], dp[i][j]);
                    // 置き換え
                    dp[i + 1][j + 1] = Math.Min(dp[i + 1][j + 1], dp[i][j] + cn[i]);
                    // 削除
                    dp[i + 1][j] = Math.Min(dp[i + 1][j], dp[i][j] + dn[i]);
                }
            }

        Console.WriteLine(dp[n][0]);
    }
}
