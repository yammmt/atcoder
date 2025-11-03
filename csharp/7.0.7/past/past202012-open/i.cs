using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main()
    {
        const int UNVISITED = int.MaxValue;

        var nmk = Console.ReadLine().Split();
        var n = int.Parse(nmk[0]);
        var m = int.Parse(nmk[1]);
        var k = int.Parse(nmk[2]);
        var hn = Console.ReadLine().Split().Select(int.Parse).ToArray();
        var ck = Console.ReadLine().Split().Select(s => int.Parse(s) - 1).ToArray();

        var edges = new List<List<int>>();
        for (int i = 0; i < n; i++)
            edges.Add(new List<int>());
        for (int i = 0; i < m; i++)
        {
            var ab = Console.ReadLine().Split();
            var a = int.Parse(ab[0]) - 1;
            var b = int.Parse(ab[1]) - 1;
            // a/b の高低は自分で読み替える
            // 避難所を始点に逆順に辿るので, 低い方から高い方に辺を引く
            if (hn[a] < hn[b])
                edges[a].Add(b);
            else
                edges[b].Add(a);
        }

        var isC = new bool[n];
        foreach (int c in ck)
            isC[c] = true;

        var ans = new int[n];
        Array.Fill(ans, UNVISITED);

        // c を始点に, 標高低い順に辺を辿る
        // 多始点 BFS とやら
        // (idx, costSum)
        var que = new Queue<(int, int)>();
        foreach (var c in ck)
            que.Enqueue((c, 0));
        while (que.TryDequeue(out (int idx, int cost) cur))
        {
            if (ans[cur.idx] != UNVISITED)
                continue;

            ans[cur.idx] = cur.cost;
            foreach (var vNext in edges[cur.idx])
                que.Enqueue((vNext, cur.cost + 1));
        }

        foreach (int a in ans)
            if (a == UNVISITED)
                Console.WriteLine(-1);
            else
                Console.WriteLine(a);
    }
}
