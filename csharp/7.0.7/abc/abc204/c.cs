using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static bool[] bfs(int v, List<List<int>> edges)
    {
        int n = edges.Count;
        var ret = new bool[n];
        var que = new Queue<int>();
        que.Enqueue(v);
        while (que.TryDequeue(out var vCur))
        {
            if (ret[vCur]) continue;

            ret[vCur] = true;
            foreach (var vNext in edges[vCur])
            {
                que.Enqueue(vNext);
            }
        }

        return ret;
    }

    static void Main()
    {
        var nm = Console.ReadLine().Split().Select(int.Parse).ToArray();
        var n = nm[0];
        var m = nm[1];
        var edges = new List<List<int>>();
        for (int i = 0; i < n; i++)
        {
            edges.Add(new List<int>());
        }
        for (int i = 0; i < m; i++)
        {
            var ab = Console.ReadLine().Split().Select(int.Parse).ToArray();
            edges[ab[0] - 1].Add(ab[1] - 1);
        }

        int ans = 0;
        for (int i = 0; i < n; i++)
        {
            var couldVisit = bfs(i, edges);
            for (int j = 0; j < n; j++)
            {
                if (couldVisit[j]) ans++;
            }
        }

        Console.WriteLine(ans);
    }
}