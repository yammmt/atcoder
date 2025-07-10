using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main()
    {
        var n = int.Parse(Console.ReadLine());
        var an = Console.ReadLine().Split().Select(int.Parse).ToArray();

        // 最大値 1e9 だと, 一次元配列を使いたくはない
        var counter = new Dictionary<int, long>();
        foreach (var a in an)
        {
            long v = 0;
            if (counter.TryGetValue(a, out v))
            {
                counter[a] = v + 1;
            }
            else
            {
                counter.Add(a, 1);
            }
        }

        var ans = 0L;
        for (int i = 0; i < n; i++)
        {
            var v = counter[an[i]] - 1;
            counter[an[i]] = v;
            ans += (long)(n - i - 1) - v;
        }

        Console.WriteLine(ans);
    }
}