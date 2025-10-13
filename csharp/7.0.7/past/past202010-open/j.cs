using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static long[] Dijkstra(List<List<(int, long)>> edges, char[] s, int[] x3)
    {
        const long unvisited = long.MaxValue;
        var n = s.Length;
        var ret = new long[n];
        Array.Fill(ret, unvisited);

        var typeAppeared = new bool[3];
        // <町, 合計コスト>
        var pq = new PriorityQueue<int, long>();
        pq.Enqueue(0, 0);
        while (pq.TryDequeue(out int townCur, out long costCur))
        {
            if (ret[townCur] != unvisited)
                continue;

            ret[townCur] = costCur;

            // 辺を直接辿る
            foreach (var (townNext, addedCost) in edges[townCur])
            {
                if (ret[townNext] != unvisited)
                    continue;

                var costNext = costCur + addedCost;
                pq.Enqueue(townNext, costNext);
            }

            // ワープ
            // idx は 0: ab, 1: ac, 2: bc
            // a=0, b=1, c=2 として和を取ると a+b=1, a+c=2, b+c=3=0
            // 今+移動後で対応が取れるので使う, +2 して合わせる
            var typeCur = s[townCur] - 'A';
            if (typeAppeared[typeCur])
                continue;

            typeAppeared[typeCur] = true;
            for (int i = 0; i < n; i++)
            {
                var typeNext = s[i] - 'A';
                if (typeCur == typeNext || ret[i] != unvisited)
                    continue;

                var addedCost = x3[(typeCur + typeNext + 2) % 3];
                pq.Enqueue(i, costCur + addedCost);
            }
        }

        return ret;
    }

    static void Main()
    {
        var nm = Console.ReadLine().Split();
        var n = int.Parse(nm[0]);
        var m = int.Parse(nm[1]);
        var x3 = Console.ReadLine().Split().Select(int.Parse).ToArray();
        var s = Console.ReadLine().ToCharArray();

        // ワープ先のチェックは高々 6 回くらいしか実行されないので, 端折ってもよい
        var edges = new List<List<(int, long)>>();
        for (int i = 0; i < n; i++)
            edges.Add(new List<(int, long)>());
        for (int i = 0; i < m; i++)
        {
            var abc = Console.ReadLine().Split();
            var a = int.Parse(abc[0]) - 1;
            var b = int.Parse(abc[1]) - 1;
            var c = long.Parse(abc[2]);

            edges[a].Add((b, c));
            edges[b].Add((a, c));
        }

        // Dijkstra, 同じタイプのワープは高々一度しか考えなくてよい
        var ans = Dijkstra(edges, s, x3);
        Console.WriteLine(ans[n - 1]);
    }
}
