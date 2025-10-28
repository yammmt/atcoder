using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main()
    {
        var nk = Console.ReadLine().Split();
        var n = int.Parse(nk[0]);
        var k = int.Parse(nk[1]) - 1;

        var dict = new Dictionary<string, int>();
        for (int i = 0; i < n; i++)
        {
            var s = Console.ReadLine();
            if (dict.TryGetValue(s, out var v))
                dict[s] = v + 1;
            else
                dict.Add(s, 1);
        }

        var l = dict
            .OrderByDescending(p => p.Value)
            .ToList();

        var isAmbiguous = false;
        if (k > 0 && l[k].Value == l[k - 1].Value)
            isAmbiguous = true;
        if (k < l.Count - 1 && l[k].Value == l[k + 1].Value)
            isAmbiguous = true;

        if (isAmbiguous)
            Console.WriteLine("AMBIGUOUS");
        else
            Console.WriteLine(l[k].Key);
    }
}
