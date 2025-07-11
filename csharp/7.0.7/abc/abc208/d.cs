using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main()
    {
        const int UNVISITED = 1_000_000_000;

        // n <= 400 だから O(n^3) が成り立つ 多分
        var nm = Console.ReadLine().Split();
        var n = int.Parse(nm[0]);
        var m = int.Parse(nm[1]);

        var dist = new int[n, n];
        for (int i = 0; i < n; i++)
        {
            for (int j = 0; j < n; j++)
            {
                if (i == j)
                    // already 0
                    continue;

                dist[i, j] = UNVISITED;
            }
        }

        for (int i = 0; i < m; i++)
        {
            var abc = Console.ReadLine().Split();
            var a = int.Parse(abc[0]) - 1;
            var b = int.Parse(abc[1]) - 1;
            var c = int.Parse(abc[2]);
            dist[a, b] = c;
        }

        long ans = 0;
        for (int k = 0; k < n; k++)
        {
            for (int i = 0; i < n; i++)
            {
                for (int j = 0; j < n; j++)
                {
                    dist[i, j] = Math.Min(dist[i, j], dist[i, k] + dist[k, j]);
                }
            }

            for (int i = 0; i < n; i++)
            {
                for (int j = 0; j < n; j++)
                {
                    if (dist[i, j] == UNVISITED)
                        continue;

                    ans += dist[i, j];
                }
            }
        }

        Console.WriteLine(ans);
    }
}
