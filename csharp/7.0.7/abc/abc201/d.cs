using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main()
    {
        var hw = Console.ReadLine().Split().Select(int.Parse).ToArray();
        var h = hw[0];
        var w = hw[1];
        var ahw = new int[h, w];
        for (int i = 0; i < h; i++)
        {
            string line = Console.ReadLine();
            for (int j = 0; j < w; j++)
            {
                if (line[j] == '+')
                    ahw[i, j] = 1;
                else
                    ahw[i, j] = -1;
            }
        }

        // dp[i][j]: (i, j) からの移動後の takahashi - aoki 最大値
        var dp = new int[h, w];
        for (int i = h - 1; i >= 0; i--)
        {
            for (int j = w - 1; j >= 0; j--)
            {
                if ((i + 1 >= h) && (j + 1 >= w))
                {
                    continue;
                }

                if ((i + j) % 2 == 0)
                {
                    dp[i, j] = Int32.MinValue;
                    // Takahashi turn
                    if (i + 1 < h)
                        dp[i, j] = Math.Max(dp[i, j], dp[i + 1, j] + ahw[i + 1, j]);
                    if (j + 1 < w)
                        dp[i, j] = Math.Max(dp[i, j], dp[i, j + 1] + ahw[i, j + 1]);
                }
                else
                {
                    dp[i, j] = Int32.MaxValue;
                    // Aoki turn
                    if (i + 1 < h)
                        dp[i, j] = Math.Min(dp[i, j], dp[i + 1, j] - ahw[i + 1, j]);
                    if (j + 1 < w)
                        dp[i, j] = Math.Min(dp[i, j], dp[i, j + 1] - ahw[i, j + 1]);
                }
                // Console.WriteLine(i + "," + j + ": " + dp[i, j]);
            }
        }

        if (dp[0, 0] > 0)
            Console.WriteLine("Takahashi");
        else if (dp[0, 0] < 0)
            Console.WriteLine("Aoki");
        else
            Console.WriteLine("Draw");
    }
}