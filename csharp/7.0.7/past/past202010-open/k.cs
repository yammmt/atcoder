using System;
using System.Linq;

class Program
{
    static void Main()
    {
        const long mod = 1_000_000_000;
        const int aMax = 20;

        // 出現する数字は 20 種類しかない
        // すると事実上各数列内の転倒数を O(n) で数えられる
        // 数字の出現回数を数えてうまいことやれば求まる

        var k = int.Parse(Console.ReadLine());
        var inversion_nums = new long[k];
        var counters = new long[k, aMax];
        for (int i = 0; i < k; i++)
        {
            var n = int.Parse(Console.ReadLine());
            var an = Console.ReadLine().Split().Select(a => int.Parse(a) - 1).ToArray();
            for (int j = 0; j < n; j++)
            {
                for (int kk = an[j] + 1; kk < aMax; kk++)
                {
                    inversion_nums[i] += counters[i, kk];
                }
                counters[i, an[j]]++;
            }
        }

        var q = int.Parse(Console.ReadLine());
        var bq = Console.ReadLine().Split().Select(b => int.Parse(b) - 1).ToArray();
        long ans = 0;
        var counterX = new long[aMax];
        foreach (var b in bq)
        {
            // 既に登場した数字に起因する分
            for (int i = 0; i < aMax; i++)
            {
                for (int j = i + 1; j < aMax; j++)
                {
                    ans = (ans + counters[b, i] * counterX[j]) % mod;
                }
            }

            // 足す配列内部で完結する分
            ans = (ans + inversion_nums[b]) % mod;

            // 各数字の登場回数を更新
            for (int i = 0; i < aMax; i++)
                counterX[i] = (counterX[i] + counters[b, i]) % mod;
        }

        Console.WriteLine(ans);
    }
}
