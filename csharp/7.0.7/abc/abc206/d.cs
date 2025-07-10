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
        const int A_MAX = 200_001;
        var n = int.Parse(Console.ReadLine());
        var an = Console.ReadLine().Split().Select(int.Parse).ToArray();

        // 一致していない組に対して, 要素数-1 回の置換が必要

        var uf = new UnionFind(A_MAX);
        // = 不要, 中央は切って良い
        for (int i = 0; i < n / 2; i++)
        {
            uf.Union(an[i], an[n - i - 1]);
        }

        var memberNumPerGroup = new int[A_MAX];
        for (int i = 0; i < A_MAX; i++)
            memberNumPerGroup[uf.Find(i)]++;

        var ans = 0;
        foreach (var m in memberNumPerGroup)
            ans += Math.Max(m, 1) - 1;

        Console.WriteLine(ans);
    }
}