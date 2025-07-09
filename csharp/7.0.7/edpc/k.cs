using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main()
    {
        var nk = Console.ReadLine().Split();
        var n = int.Parse(nk[0]);
        var k = int.Parse(nk[1]);
        var an = Console.ReadLine().Split().Select(int.Parse).ToArray();

        // dp[i]: 石の数が残り i 個の場合に先手が勝てるなら true
        //        この定義では, 0 から回すと見た目とは異なり実態は後退解析
        // 負けの盤面を押し付けられるか, を考える
        var dp = new bool[k + 1];
        for (int i = 1; i <= k; i++)
        {
            foreach (var a in an)
            {
                var iFrom = i - a;
                if (iFrom < 0)
                    continue;

                if (!dp[iFrom])
                {
                    dp[i] = true;
                    break;
                }
            }
        }

        if (dp[k])
            Console.WriteLine("First");
        else
            Console.WriteLine("Second");
    }
}
