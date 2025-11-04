using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main()
    {
        const double DUMMY = 1.0e+20;
        (int, int)[] DIRS = new (int, int)[] { (-1, 0), (1, 0), (0, 0), (0, -1), (0, 1) };

        var s44 = new char[4][];
        for (int i = 0; i < 4; i++)
        {
            s44[i] = Console.ReadLine().ToCharArray();
        }

        uint grid = 0;
        for (uint i = 0; i < 4; i++)
        {
            for (uint j = 0; j < 4; j++)
            {
                if (s44[i][j] == '#')
                {
                    grid |= 1u << (int)(i * 4u + j);
                }
            }
        }

        var dp = new double[1 << 16];
        Array.Fill(dp, DUMMY);
        dp[0] = 0.0;

        // マスすべてあてた状態から始めて, 全盤面に到達するまでの最小手数を算出する
        // 算出した値の中で入力盤面に一致するものが答え
        // 貰う DP
        for (int s = 0; s < (1 << 16); s++)
        {
            for (int i = 0; i < 4; i++)
            {
                for (int j = 0; j < 4; j++)
                {
                    var s2s = new List<int>();
                    foreach (var dir in DIRS)
                    {
                        var iNext = i + dir.Item1;
                        var jNext = j + dir.Item2;
                        if (iNext < 0 || iNext >= 4 || jNext < 0 || jNext >= 4)
                        {
                            continue;
                        }

                        // 同じ盤面からの遷移は省く
                        // ss: 今見ているマスを的でないとしたもの
                        var ss = s & ~(1 << (iNext * 4 + jNext));
                        if (ss != s)
                        {
                            s2s.Add(ss);
                        }
                    }

                    if (s2s.Count == 0)
                    {
                        continue;
                    }

                    var dpSum = 0.0;
                    foreach (var ss in s2s)
                    {
                        dpSum += dp[ss];
                    }
                    dp[s] = Math.Min((dpSum + 5.0) / (double)s2s.Count, dp[s]);
                }
            }
        }

        Console.WriteLine(string.Format("{0:f8}", dp[grid]));
    }
}
