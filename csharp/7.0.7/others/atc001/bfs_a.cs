using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    const int Unvisited = 1_000_000_000;

    private readonly record struct Vertex(int X, int Y);

    static readonly Vertex[] dir =
    {
        new Vertex(-1, 0),
        new Vertex(1, 0),
        new Vertex(0, -1),
        new Vertex(0, 1)
    };

    static int[,] bfs(Vertex s, string[] chw)
    {
        int h = chw.Length;
        int w = chw[0].Length;

        int[,] costs = new int[h, w];
        for (int i = 0; i < h; i++)
        {
            for (int j = 0; j < w; j++)
            {
                costs[i, j] = Unvisited;
            }
        }

        var que = new Queue<(Vertex, int)>();
        que.Enqueue((s, 0));
        while (que.TryDequeue(out var cur))
        {
            var (v_cur, cost_cur) = cur;
            if (costs[v_cur.X, v_cur.Y] != Unvisited)
            {
                continue;
            }

            costs[v_cur.X, v_cur.Y] = cost_cur;
            foreach (var d in dir)
            {
                var x_next = v_cur.X + d.X;
                var y_next = v_cur.Y + d.Y;
                if (x_next < 0 || y_next < 0 || x_next >= h || y_next >= w || chw[x_next][y_next] == '#')
                {
                    continue;
                }

                que.Enqueue((new Vertex(x_next, y_next), cost_cur + 1));
            }
        }

        return costs;
    }

    static void Main(string[] args)
    {
        int[] hw = Console.ReadLine().Split().Select(int.Parse).ToArray();
        var h = hw[0];
        var w = hw[1];
        string[] chw = new string[h];
        for (int i = 0; i < h; i++)
        {
            chw[i] = Console.ReadLine();
        }

        Vertex s = new Vertex(0, 0);
        Vertex g = new Vertex(0, 0);
        for (int i = 0; i < h; i++)
        {
            for (int j = 0; j < w; j++)
            {
                if (chw[i][j] == 's')
                {
                    s = new Vertex(i, j);
                }
                else if (chw[i][j] == 'g')
                {
                    g = new Vertex(i, j);
                }
            }
        }

        var costs = bfs(s, chw);
        Console.WriteLine(costs[g.X, g.Y] == Unvisited ? "No" : "Yes");
    }
}