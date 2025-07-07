using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main()
    {
        var n = int.Parse(Console.ReadLine());
        var an = Console.ReadLine().Split().Select(s => int.Parse(s) % 200).ToArray();

        var counter = new int[200];
        foreach (int a in an)
            counter[a]++;

        long ans = 0;
        foreach (int a in an)
        {
            counter[a]--;
            ans += (long)counter[a];
        }

        Console.WriteLine(ans);
    }
}