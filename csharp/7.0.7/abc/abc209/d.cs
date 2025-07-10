using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static int[] bfs(List<List<int>> edge)
    {
        var n = edge.Count;
        var ret = new int[n];
        var visited = new bool[n];
        var que = new Queue<(int, int)>();
        que.Enqueue((0, 0));
        while (que.TryDequeue(out var v))
        {
            var (vCur, costCur) = v;

            if (visited[vCur])
                continue;

            visited[vCur] = true;
            ret[vCur] = costCur;

            foreach (var vNext in edge[vCur])
                que.Enqueue((vNext, costCur + 1));
        }

        return ret;
    }

    static void Main()
    {
        var nq = Console.ReadLine().Split();
        var n = int.Parse(nq[0]);
        var q = int.Parse(nq[1]);

        var edge = new List<List<int>>();
        for (int i = 0; i < n; i++)
            edge.Add(new List<int>());

        for (int i = 0; i < n - 1; i++)
        {
            var ab = Console.ReadLine().Split();
            var a = int.Parse(ab[0]) - 1;
            var b = int.Parse(ab[1]) - 1;
            edge[a].Add(b);
            edge[b].Add(a);
        }

        // 二点間の最短距離の偶奇
        // 適当な点を始点としてしまってよい
        var dist = bfs(edge);

        for (int i = 0; i < q; i++)
        {
            var cd = Console.ReadLine().Split();
            var c = int.Parse(cd[0]) - 1;
            var d = int.Parse(cd[1]) - 1;
            if (Math.Abs(dist[c] - dist[d]) % 2 == 0)
                Console.WriteLine("Town");
            else
                Console.WriteLine("Road");
        }
    }
}
