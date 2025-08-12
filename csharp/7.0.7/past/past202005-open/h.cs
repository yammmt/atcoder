using System;
using System.Linq;

class Program
{
    static void Main()
    {
        const int dummy = 1_000_000_000;
        var nl = Console.ReadLine().Split(' ');
        var n = int.Parse(nl[0]);
        var l = int.Parse(nl[1]);
        var xn = Console.ReadLine().Split(' ').Select(int.Parse).ToArray();
        var t3 = Console.ReadLine().Split(' ').Select(int.Parse).ToArray();

        // 処理が楽なので bool 配列を用意して障害物情報を入れる
        var hasHurdle = new bool[l + 1];
        foreach (int x in xn)
            hasHurdle[x] = true;

        // 0.5 があると反射的に x2 したくなるが, しても変わらないような
        // 最短時間を管理するとジャンプ中判定が難しくなるので, DP 配列二つ持つなど
        var dp = new (int land, int air)[l + 1];
        for (int i = 0; i < l + 1; i++)
            dp[i] = (dummy, dummy);
        dp[0].land = 0;

        for (int i = 0; i < l; i++)
        {
            var pena = hasHurdle[i] ? t3[2] : 0;

            // act 1
            dp[i + 1].land = Math.Min(dp[i + 1].land, dp[i].land + t3[0] + pena);

            // act 2
            dp[i + 1].air = Math.Min(dp[i + 1].air, dp[i].land + t3[0] / 2 + t3[1] / 2 + pena);
            if (i + 2 < l + 1)
                dp[i + 2].land = Math.Min(dp[i + 2].land, dp[i].land + t3[0] + t3[1] + pena);

            // act 3
            // i + 1 は act 2 と同じ
            if (i + 2 < l + 1)
                dp[i + 2].air = Math.Min(dp[i + 2].air, dp[i].land + t3[0] / 2 + t3[1] * 3 / 2 + pena);
            if (i + 3 < l + 1)
                dp[i + 3].air = Math.Min(dp[i + 3].air, dp[i].land + t3[0] / 2 + t3[1] * 5 / 2 + pena);
            if (i + 4 < l + 1)
                dp[i + 4].land = Math.Min(dp[i + 4].land, dp[i].land + t3[0] + t3[1] * 3 + pena);
        }

        Console.WriteLine(Math.Min(dp[l].land, dp[l].air));
    }
}
