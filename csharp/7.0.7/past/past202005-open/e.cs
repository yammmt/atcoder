using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main()
    {
        var nmq = Console.ReadLine().Split(' ');
        var n = int.Parse(nmq[0]);
        var m = int.Parse(nmq[1]);
        var q = int.Parse(nmq[2]);
        var uvm = new (int u, int v)[m];
        for (int i = 0; i < m; i++)
        {
            var uv = Console.ReadLine().Split(' ').Select(int.Parse).ToArray();
            uvm[i] = (uv[0] - 1, uv[1] - 1);
        }
        var cn = Console.ReadLine().Split(' ').Select(int.Parse).ToArray();

        var edges = new List<List<int>>();
        for (int i = 0; i < n; i++)
        {
            edges.Add(new List<int>());
        }
        foreach (var uv in uvm)
        {
            edges[uv.u].Add(uv.v);
            edges[uv.v].Add(uv.u);
        }

        for (int i = 0; i < q; i++)
        {
            var s = Console.ReadLine().Split(' ').Select(int.Parse).ToArray();
            var x = s[1] - 1;
            Console.WriteLine(cn[x]);
            if (s[0] == 1)
            {
                foreach (var v in edges[x])
                {
                    cn[v] = cn[x];
                }
            }
            else
            {
                cn[x] = s[2];
            }
        }
    }
}
