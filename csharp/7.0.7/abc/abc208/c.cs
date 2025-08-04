using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main()
    {
        var nk = Console.ReadLine().Split(' ').Select(long.Parse).ToArray();
        long n = nk[0];
        long k = nk[1];
        var an = Console.ReadLine().Split(' ').Select(int.Parse).ToArray();

        var aAndI = new (int a, int i)[n];
        for (int i = 0; i < n; i++)
        {
            aAndI[i] = (an[i], i);
        }
        Array.Sort(aAndI);

        var ans = new long[n];
        for (int i = 0; i < n; i++)
        {
            ans[i] = k / n;
        }
        for (int i = 0; i < k % n; i++)
        {
            ans[aAndI[i].i] += 1;
        }

        for (int i = 0; i < n; i++)
        {
            Console.WriteLine(ans[i]);
        }
    }
}
