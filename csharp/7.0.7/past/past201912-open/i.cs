using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main(string[] args)
    {
        string[] s = Console.ReadLine().Split(' ');
        int n = int.Parse(s[0]);
        int m = int.Parse(s[1]);

        (int, long)[] scm = new (int, long)[m];
        for (int i = 0; i < m; i++)
        {
            string[] ss = Console.ReadLine().Split(' ');
            char[] c = ss[0].ToCharArray();
            int bit = 0;
            for (int j = 0; j < n; j++)
            {
                if (c[j] == 'Y')
                {
                    bit += (1 << j);
                }
            }

            scm[i] = (bit, long.Parse(ss[1]));
        }

        int dpLen = 1 << n;
        var dp = new long[dpLen];
        Array.Fill(dp, 1_000_000_000_000L);
        dp[0] = 0;

        for (int i = 0; i < dpLen; i++)
        {
            // dummy 避けはあった方が高速な気もするが
            for (int j = 0; j < m; j++)
            {
                int sNext = i | scm[j].Item1;
                dp[sNext] = Math.Min(dp[sNext], dp[i] + scm[j].Item2);
            }
        }

        if (dp[dpLen - 1] == 1_000_000_000_000L)
        {
            Console.WriteLine("-1");
        }
        else
        {
            Console.WriteLine(dp[dpLen - 1]);
        }
    }
}
