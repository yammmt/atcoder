using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main(string[] args)
    {
        var n = int.Parse(Console.ReadLine());
        int[] a = Console.ReadLine().Split().Select(int.Parse).ToArray();
        var aLen = (int)Math.Pow(2, n);

        var winners = Enumerable.Range(0, aLen)
            .Select(i =>
            {
                return (idx: i, strength: a[i]);
            }).ToArray();
        var ans = new int[aLen];

        while (winners.Count() > 1)
        {
            var winners_next = new (int, int)[winners.Count() / 2];
            int i = 0;
            while (i < winners.Count())
            {
                ans[winners[i].idx]++;
                ans[winners[i + 1].idx]++;
                if (winners[i].strength > winners[i + 1].strength)
                {
                    winners_next[i / 2] = winners[i];
                }
                else
                {
                    winners_next[i / 2] = winners[i + 1];
                }
                i += 2;
            }
            winners = winners_next;
        }

        foreach (var v in ans)
        {
            Console.WriteLine(v);
        }
    }
}
