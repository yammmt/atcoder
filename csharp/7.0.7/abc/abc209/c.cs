using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main()
    {
        const long MOD = 1_000_000_007;

        var n = int.Parse(Console.ReadLine());
        var cn = Console.ReadLine().Split().Select(long.Parse).ToArray();
        Array.Sort(cn);

        long ans = 1;
        for (int i = 0; i < n; i++)
        {
            if (i > cn[i])
            {
                ans = 0;
                break;
            }

            ans = (ans * (cn[i] - i)) % MOD;
        }

        Console.WriteLine(ans);
    }
}
