using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static int Gcd(int a, int b)
    {
        if (a % b == 0)
        {
            return b;
        }

        var c = a % b;
        return Gcd(b, c);
    }

    static bool CanRing(int range, int d, int n)
    {
        // 初回観測で +1, 以後は閉区間で d 経過後
        return range / d + 1 <= n;
    }

    static void Main()
    {
        var nk = Console.ReadLine().Split();
        var n = int.Parse(nk[0]);
        var k = int.Parse(nk[1]);
        var ak = Console.ReadLine().Split().Select(int.Parse).ToArray();

        // d である可能性があるのは, 鳴った時間の差分最小値の因数
        // というより, 隣との時間差について GCD を求めればよい
        // "N 回鳴った" の部分から, 約数小さい側に制約が入る
        // これは区間を割った数で求めればよい…はず

        var dMax = ak[1] - ak[0];
        for (int i = 2; i < k; i++)
        {
            dMax = Gcd(dMax, ak[i] - ak[i - 1]);
        }

        var ans = new List<int>();
        var range = ak[k - 1] - ak[0];
        var j = 1;
        while (j * j <= dMax)
        {
            if (dMax % j == 0)
            {
                if (CanRing(range, j, n))
                {
                    ans.Add(j);
                }
                var jj = dMax / j;
                if (jj != j && CanRing(range, jj, n))
                {
                    ans.Add(jj);
                }
            }

            j++;
        }

        ans.Sort();
        Console.WriteLine(ans.Count);
        Console.WriteLine(string.Join(" ", ans));
    }
}
