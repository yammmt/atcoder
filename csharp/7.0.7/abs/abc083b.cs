using System;
using System.Linq;

class Program
{
    static void Main(string[] args)
    {
        int[] nab = Console.ReadLine().Split(' ').Select(int.Parse).ToArray();
        int n = nab[0];
        int a = nab[1];
        int b = nab[2];

        int ans = 0;
        for (int i = 1; i <= n; i++)
        {
            int cur = 0;
            int ii = i;
            while (ii > 0)
            {
                cur += ii % 10;
                ii /= 10;
            }
            if (cur >= a && cur <= b)
            {
                ans += i;
            }
        }

        Console.WriteLine(ans);
    }
}
