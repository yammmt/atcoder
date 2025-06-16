using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    public class UnionFind
    {
        private int[] groups;
        private int[] ranks;

        public UnionFind(int n)
        {
            groups = new int[n];
            ranks = new int[n];
            for (int i = 0; i < n; i++)
            {
                groups[i] = i;
                ranks[i] = 1;
            }
        }

        public bool Equiv(int a, int b)
        {
            return Find(a) == Find(b);
        }

        public int Find(int n)
        {
            if (groups[n] == n)
            {
                return n;
            }
            else
            {
                groups[n] = Find(groups[n]);
                return groups[n];
            }
        }

        public void Union(int a, int b)
        {
            int x = Find(a);
            int y = Find(b);
            if (x == y)
            {
                return;
            }

            if (ranks[x] < ranks[y])
            {
                groups[x] = y;
            }
            else
            {
                groups[y] = x;
                if (ranks[x] == ranks[y]) ranks[x]++;
            }
        }
    }

    static void Main()
    {
        var nq = Console.ReadLine().Split().Select(int.Parse).ToArray();
        var n = nq[0];
        var q = nq[1];

        var uf = new UnionFind(n);
        for (int i = 0; i < q; i++)
        {
            var query = Console.ReadLine().Split().Select(int.Parse).ToArray();
            var p = query[0];
            var a = query[1];
            var b = query[2];
            if (p == 0)
            {
                // 連結
                uf.Union(a, b);
            }
            else
            {
                // 判定
                Console.WriteLine(uf.Equiv(a, b) ? "Yes" : "No");
            }
        }
    }
}