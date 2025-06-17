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

    static double Distance((double x, double y, int c) a, (double x, double y, int c) b)
    {
        var dx = a.x - b.x;
        var dy = a.y - b.y;
        var ret = Math.Sqrt(dx * dx + dy * dy);
        if (a.c != b.c) ret *= 10.0;
        return ret;
    }

    static void Main(string[] args)
    {
        var nm = Console.ReadLine().Split().Select(int.Parse).ToArray();
        var n = nm[0];
        var m = nm[1];
        var bigs = new List<(double x, double y, int c)>();
        for (int i = 0; i < n; i++)
        {
            var xyc = Console.ReadLine().Split().Select(int.Parse).ToArray();
            bigs.Add(((double)xyc[0], (double)xyc[1], xyc[2]));
        }
        var smalls = new List<(double x, double y, int c)>();
        for (int i = 0; i < m; i++)
        {
            var xyc = Console.ReadLine().Split().Select(int.Parse).ToArray();
            smalls.Add(((double)xyc[0], (double)xyc[1], xyc[2]));
        }

        // 使う小さな等のパターンを全探索
        var ans = double.MaxValue;
        for (int b = 0; b < (1 << m); b++)
        {
            int bit = b;
            bool[] isSmallUsed = new bool[m];
            for (int i = 0; i < m; i++)
            {
                if (bit % 2 == 1) isSmallUsed[i] = true;
                bit /= 2;
            }

            var score = 0.0;
            // (頂点, 頂点), cost
            var pq = new PriorityQueue<(int, int), double>();
            // 大きな塔同士
            for (int i = 0; i < n; i++)
            {
                for (int j = i + 1; j < n; j++)
                {
                    var cost = Distance(bigs[i], bigs[j]);
                    pq.Enqueue((i, j), cost);
                }
            }
            // 小さな塔同士
            for (int i = 0; i < m; i++)
            {
                if (!isSmallUsed[i]) continue;

                for (int j = i + 1; j < m; j++)
                {
                    if (!isSmallUsed[j]) continue;

                    var cost = Distance(smalls[i], smalls[j]);
                    pq.Enqueue((i + n, j + n), cost);
                }
            }
            // 大小
            for (int i = 0; i < n; i++)
            {
                for (int j = 0; j < m; j++)
                {
                    if (!isSmallUsed[j]) continue;

                    var cost = Distance(bigs[i], smalls[j]);
                    pq.Enqueue((i, j + n), cost);
                }
            }

            var uf = new UnionFind(n + m);
            while (pq.TryDequeue(out var vv, out var cost))
            {
                // 枝刈りした方が高速だが, 妥協しても計算量的には問題ない上に実装は楽
                var v0 = vv.Item1;
                var v1 = vv.Item2;
                if (uf.Equiv(v0, v1)) continue;

                uf.Union(v0, v1);
                score += cost;
            }

            ans = Math.Min(ans, score);
        }

        Console.WriteLine(ans);
    }
}
