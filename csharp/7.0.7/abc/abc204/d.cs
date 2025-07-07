using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main()
    {
        int n = int.Parse(Console.ReadLine());
        var tn = Console.ReadLine().Split().Select(int.Parse).ToArray();

        int tnSum = tn.Sum();
        var dp = new bool[tnSum + 1];
        dp[0] = true;
        foreach (int t in tn)
        {
            bool[] dpNxt = dp.ToArray();
            for (int i = 0; i < tnSum + 1; i++)
            {
                if (dp[i])
                {
                    dpNxt[i + t] = true;
                }
            }
            dp = dpNxt;
        }

        int ans = tnSum + 1;
        for (int i = 0; i < tnSum + 1; i++)
        {
            if (dp[i])
                ans = Math.Min(ans, Math.Max(i, tnSum - i));
        }

        Console.WriteLine(ans);
    }
}