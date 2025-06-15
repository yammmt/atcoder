using System;
using System.Linq;

class Program
{
    private readonly record struct Vertex(int X, int Y);

    static readonly Vertex[] dir = {
        new Vertex(-1, 0),
        new Vertex(1, 0),
        new Vertex(0, -1),
        new Vertex(0, 1)
    };

    static bool Dfs(Vertex v_cur, Vertex v_goal, string[] chw, bool[,] visited)
    {
        if (v_cur == v_goal)
        {
            return true;
        }

        visited[v_cur.X, v_cur.Y] = true;
        int h = chw.Length;
        int w = chw[0].Length;
        foreach (Vertex d in dir)
        {
            int x_next = v_cur.X + d.X;
            int y_next = v_cur.Y + d.Y;
            if (x_next < 0 || y_next < 0 || x_next >= h || y_next >= w || visited[x_next, y_next] || chw[x_next][y_next] == '#')
            {
                continue;
            }

            Vertex v_next = new Vertex(x_next, y_next);
            if (Dfs(v_next, v_goal, chw, visited))
            {
                return true;
            }
        }

        return false;
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

        bool[,] visited = new bool[h, w];
        Console.WriteLine(Dfs(s, g, chw, visited) ? "Yes" : "No");
    }
}
