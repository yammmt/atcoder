using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main()
    {
        var nk = Console.ReadLine().Split(' ').Select(int.Parse).ToArray();
        var n = nk[0];
        var k = nk[1];
        var cn = Console.ReadLine().Split(' ').Select(int.Parse).ToArray();

        // TODO: use `Dictionary<int, int>`

        var ht = new Hashtable();
        for (int i = 0; i < k; i++)
        {
            var added = cn[i];
            if (ht.ContainsKey(added))
            {
                ht[added] = (int)ht[added] + 1;
            }
            else
            {
                ht[added] = 1;
            }
        }

        var ans = ht.Count;
        for (int i = k; i < n; i++)
        {
            var removed = cn[i - k];
            if ((int)ht[removed] == 1)
            {
                ht.Remove(removed);
            }
            else
            {
                ht[removed] = (int)ht[removed] - 1;
            }

            var added = cn[i];
            if (ht.ContainsKey(added))
            {
                ht[added] = (int)ht[added] + 1;
            }
            else
            {
                ht[added] = 1;
            }

            ans = Math.Max(ans, ht.Count);
        }

        Console.WriteLine(ans);
    }
}

