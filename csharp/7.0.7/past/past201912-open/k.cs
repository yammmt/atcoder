using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static (int pre, int post)[] EulerTour(int root, List<List<int>> edges)
    {
        int time = 0;
        var ret = new (int pre, int post)[edges.Count];

        void Dfs(int v_cur, int v_prev)
        {
            ret[v_cur].pre = time++;
            foreach (int v_next in edges[v_cur])
            {
                if (v_next != v_prev)
                {
                    Dfs(v_next, v_cur);
                }
            }
            ret[v_cur].post = time++;
        }

        Dfs(root, -1);

        return ret;
    }

    static void Main(string[] args)
    {
        var n = int.Parse(Console.ReadLine());
        var pn = Enumerable.Range(0, n)
            .Select(_ => int.Parse(Console.ReadLine()) - 1)
            .ToArray();
        var q = int.Parse(Console.ReadLine());
        var abq = Enumerable.Range(0, q)
            .Select(_ =>
            {
                var ab = Console.ReadLine().Split().Select(int.Parse).ToArray();
                return (a: ab[0] - 1, b: ab[1] - 1);
            }).ToArray();

        // 上司から部下を辿るため, 向きを逆にする
        var edges = Enumerable.Range(0, n).Select(_ => new List<int>()).ToList();
        int president = -1;
        for (int i = 0; i < n; i++)
        {
            if (pn[i] == -2)
            {
                president = i;
                continue;
            }

            edges[pn[i]].Add(i);
        }

        var ranks = EulerTour(president, edges);

        foreach (var (a, b) in abq)
        {
            // a が b の部下 = b は a の上司
            // 上司 in -> 部下 in -> 部下 out -> 上司 out
            bool isSubordinate = ranks[b].pre < ranks[a].pre && ranks[a].post < ranks[b].post;
            Console.WriteLine(isSubordinate ? "Yes" : "No");
        }
    }
}
