using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main()
    {
        const int IN_MAX = 100_001;

        var n = int.Parse(Console.ReadLine());
        var an = Console.ReadLine().Split().Select(int.Parse).ToArray();
        var bn = Console.ReadLine().Split().Select(int.Parse).ToArray();
        var cn = Console.ReadLine().Split().Select(int.Parse).ToArray();

        var counter = new int[IN_MAX];
        foreach (int c in cn)
        {
            counter[bn[c - 1]]++;
        }

        long ans = 0;
        foreach (int a in an)
            ans += counter[a];

        Console.WriteLine(ans);
    }
}