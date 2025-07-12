using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main()
    {
        var n = int.Parse(Console.ReadLine());
        var an = Console.ReadLine().Split().Select(long.Parse).ToArray();

        long aMax = 0;
        foreach (var a in an)
            aMax = Math.Max(aMax, a);

        for (int i = 0; i < n; i++)
        {
            Console.Write((1_000_000_000 * an[i] + aMax / 2) / aMax);

            if (i == n - 1)
                Console.WriteLine();
            else
                Console.Write(" ");
        }
    }
}