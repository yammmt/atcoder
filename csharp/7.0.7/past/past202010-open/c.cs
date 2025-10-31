using System;
using System.Linq;

class Program
{
    static void Main()
    {
        var nm = Console.ReadLine().Split();
        var n = int.Parse(nm[0]);
        var m = int.Parse(nm[1]);

        var snm = new char[n][];
        for (int i = 0; i < n; i++)
        {
            snm[i] = Console.ReadLine().ToCharArray();
        }

        for (int i = 0; i < n; i++)
        {
            for (int j = 0; j < m; j++)
            {
                int ans = 0;
                for (int di = -1; di <= 1; di++)
                {
                    var ci = i + di;
                    if (ci < 0 || ci >= n)
                        continue;

                    for (int dj = -1; dj <= 1; dj++)
                    {
                        var cj = j + dj;
                        if (cj < 0 || cj >= m)
                            continue;

                        if (snm[ci][cj] == '#')
                            ans++;
                    }
                }
                Console.Write(ans);
            }
            Console.WriteLine();
        }
    }
}
