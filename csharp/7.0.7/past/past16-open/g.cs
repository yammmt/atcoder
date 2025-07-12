using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static bool IsTriangle(int a, int b, int c)
    {
        var ab = a + b;
        var bc = b + c;
        var ca = c + a;
        return ab > c && bc > a && ca > b;
    }

    static int Dfs(int[] a3n)
    {
        var n = a3n.Length;
        if (n == 0)
            return 1;

        int ret = 0;
        for (int i = 1; i < n; i++)
        {
            for (int j = i + 1; j < n; j++)
            {
                if (!IsTriangle(a3n[0], a3n[i], a3n[j]))
                    continue;

                var an = new int[n - 3];
                int ii = 0;
                for (int k = 1; k < n; k++)
                {
                    if (k != i && k != j)
                        an[ii++] = a3n[k];
                }
                ret += Dfs(an);
            }
        }

        return ret;
    }

    static void Main()
    {
        var n = int.Parse(Console.ReadLine());
        var a3n = Console.ReadLine().Split().Select(int.Parse).ToArray();

        // 三本一組に分ける, nC3 を数えるではなく
        Console.Write(Dfs(a3n));
    }
}