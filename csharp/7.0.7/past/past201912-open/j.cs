using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    const int Dummy = 1_000_000_000;

    private static int[,] MinCost(int h_begin, int w_begin, int[,] ahw)
    {
        (int, int)[] dir = { (-1, 0), (0, -1), (1, 0), (0, 1) };
        int h = ahw.GetLength(0);
        int w = ahw.GetLength(1);
        var ret = new int[h, w];
        // TODO: 初期化が汚い
        for (int i = 0; i < h; i++)
        {
            for (int j = 0; j < w; j++)
            {
                ret[i, j] = Dummy;
            }
        }
        // エラー, わからん
        // Array.Initialize(ret, 1_000_000_000);

        var pq = new PriorityQueue<(int, int), int>();
        pq.Enqueue((h_begin, w_begin), 0);
        while (pq.TryDequeue(out (int, int) v, out int cost))
        {
            int h_cur = v.Item1;
            int w_cur = v.Item2;
            if (ret[h_cur, w_cur] != Dummy)
            {
                continue;
            }

            ret[h_cur, w_cur] = cost;
            foreach ((int, int) d in dir)
            {
                int h_next = h_cur + d.Item1;
                int w_next = w_cur + d.Item2;
                if (h_next < 0 || h_next > h - 1 || w_next < 0 || w_next > w - 1 || ret[h_next, w_next] != Dummy)
                {
                    continue;
                }

                pq.Enqueue((h_next, w_next), cost + ahw[h_next, w_next]);
            }
        }

        return ret;
    }

    static void Main(string[] args)
    {
        int[] hw = Console.ReadLine().Split(' ').Select(int.Parse).ToArray();
        int h = hw[0];
        int w = hw[1];
        var ahw = new int[h, w];
        for (int i = 0; i < h; i++)
        {
            int[] a = Console.ReadLine().Split(' ').Select(int.Parse).ToArray();
            for (int j = 0; j < w; j++)
            {
                ahw[i, j] = a[j];
            }
        }

        // 中間地点を設定する
        // 左下 -> 中間 -> 右下 -> 中間 -> 右上
        // これらの経路は中継点以外で重ならない

        int[,] to_lb = MinCost(h - 1, 0, ahw);
        int[,] to_rb = MinCost(h - 1, w - 1, ahw);
        int[,] to_rt = MinCost(0, w - 1, ahw);

        int ans = Dummy;
        for (int i = 0; i < h; i++)
        {
            for (int j = 0; j < w; j++)
            {
                int cost = to_lb[i, j] + to_rb[i, j] + to_rt[i, j] - 2 * ahw[i, j];
                ans = Math.Min(ans, cost);
            }
        }

        Console.WriteLine(ans);
    }
}
