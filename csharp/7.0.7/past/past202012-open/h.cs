using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;

class Program
{
    static void Main()
    {
        (int, int)[] DIRS = new (int, int)[] { (-1, 0), (1, 0), (0, -1), (0, 1) };


        var hw = Console.ReadLine().Split();
        var h = int.Parse(hw[0]);
        var w = int.Parse(hw[1]);
        var rc = Console.ReadLine().Split();
        var r = int.Parse(rc[0]) - 1;
        var c = int.Parse(rc[1]) - 1;
        var shw = new char[h][];
        for (int i = 0; i < h; i++)
            shw[i] = Console.ReadLine().ToCharArray();

        // 逆に辺を貼って終点から BFS

        var visited = new bool[h, w];
        var que = new Queue<(int, int)>();
        que.Enqueue((r, c));
        while (que.TryDequeue(out var curPos))
        {
            var xCur = curPos.Item1;
            var yCur = curPos.Item2;
            if (visited[xCur, yCur])
                continue;

            visited[xCur, yCur] = true;
            foreach (var dir in DIRS)
            {
                var xNext = xCur + dir.Item1;
                var yNext = yCur + dir.Item2;
                if (xNext < 0 || xNext >= h || yNext < 0 || yNext >= w || visited[xNext, yNext])
                    continue;

                if (
                    shw[xNext][yNext] == '.'
                    || (dir == (-1,  0) && shw[xNext][yNext] == 'v')
                    || (dir == ( 1,  0) && shw[xNext][yNext] == '^')
                    || (dir == ( 0, -1) && shw[xNext][yNext] == '>')
                    || (dir == ( 0,  1) && shw[xNext][yNext] == '<')
                )
                {
                    que.Enqueue((xNext, yNext));
                }
            }
        }

        // 愚直に一文字ずつ出力させると TLE
        var ans = new StringBuilder(h * (w + 1));
        for (int i = 0; i < h; i++)
        {
            for (int j = 0; j < w; j++)
            {
                if (shw[i][j] == '#')
                    ans.Append('#');
                else if (visited[i, j])
                    ans.Append('o');
                else
                    ans.Append('x');
            }
            ans.Append('\n');
        }
        Console.Write(ans.ToString());
    }
}
