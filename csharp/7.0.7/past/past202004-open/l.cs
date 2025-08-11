using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main(string[] args)
    {
        var nkd = Console.ReadLine().Split(' ').ToArray();
        var n = int.Parse(nkd[0]);
        var k = int.Parse(nkd[1]);
        var d = int.Parse(nkd[2]);
        var an = Console.ReadLine().Split(' ').Select(int.Parse).ToArray();

        // 頭から見て選択可能な数字のうちの最小値を使う, を繰り返すと, ヒープ考慮し O(nlogn)
        // 一度でも選択可能になったものを全部ヒープに入れて, 抽出時に使用不可値を捨てればよい
        // のだが C# の標準ライブラリに軽いヒープはなさそうで, ちょっと大げさな書き方になる

        // (idx, (an[idx], idx))
        var pq = new PriorityQueue<int, (int, int)>();
        // 後ろから k * (残り要素数 - 1) 個は選択不可
        // Max の意味上, 閉区間とする
        var iMax = n - d * (k - 1) - 1;
        if (iMax < 0)
        {
            Console.WriteLine("-1");
            return;
        }

        var iMin = 0;
        var ans = new int[k];
        for (int i = iMin; i <= iMax; i++)
            pq.Enqueue(i, (an[i], i));
        for (int i = 0; i < k; i++)
        {
            var iUsed = 0;
            while (pq.TryDequeue(out int _, out (int a, int i) pr))
            {
                // Console.WriteLine("dq: [" + pr.i + "]");
                if (pr.i >= iMin)
                {
                    // Console.WriteLine("  pass");
                    ans[i] = pr.a;
                    iUsed = pr.i;
                    break;
                }
            }

            if (i == k - 1)
                break;

            var iMaxBefore = iMax;
            // n - d * (残り要素数 - 1) - 1
            // 残り要素数 - 1: 例えば k=2 に対し i=0 として 1 個
            //   最後の 1 個は全領域から選べるので iMax=n とすべく () の中を 0 にする
            // -1" 閉区間で見ているので origin を合わせる
            iMax = n - d * (k - i - 1 - 1) - 1;
            for (int j = iMaxBefore + 1; j <= iMax; j++)
            {
                pq.Enqueue(j, (an[j], j));
            }
            iMin = iUsed + d;
        }

        Console.WriteLine(string.Join(" ", ans));
    }
}
