using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main()
    {
        const int GRID_SIZE = 410;
        const int GRID_OFFSET = 202;
        const int UNVISITED = 1_000_000_000;
        // const 付けたい
        (int, int)[] dir = {
            (1, 1),
            (0, 1),
            (-1, 1),
            (1, 0),
            (-1, 0),
            (0, -1),
        };

        var obstacles = new HashSet<(int, int)>();

        bool CanMove(int x, int y) =>
            !(x < 0 || x >= GRID_SIZE || y < 0 || y >= GRID_SIZE || obstacles.Contains((x, y)));


        var nxy = Console.ReadLine().Split(' ');
        var n = int.Parse(nxy[0]);
        var x = int.Parse(nxy[1]) + GRID_OFFSET;
        var y = int.Parse(nxy[2]) + GRID_OFFSET;
        for (int i = 0; i < n; i++)
        {
            var xy = Console.ReadLine().Split(' ');
            obstacles.Add((int.Parse(xy[0]) + GRID_OFFSET, int.Parse(xy[1]) + GRID_OFFSET));
        }

        var scores = new int[GRID_SIZE][];
        for (int i = 0; i < GRID_SIZE; i++)
        {
            scores[i] = new int[GRID_SIZE];
            ARray.Fill(scores[i], UNVISITED);
        }

        var que = new Queue<(int x, int y, int score)>();
        que.Enqueue((GRID_OFFSET, GRID_OFFSET, 0));
        while (que.TryDequeue(out var cur))
        {
            if (scores[cur.x][cur.y] != UNVISITED)
            {
                continue;
            }

            scores[cur.x][cur.y] = cur.score;

            foreach (var (dx, dy) in dir)
            {
                var xNext = cur.x + dx;
                var yNext = cur.y + dy;
                if (CanMove(xNext, yNext))
                {
                    que.Enqueue((xNext, yNext, cur.score + 1));
                }
            }
        }

        if (scores[x][y] == UNVISITED)
        {
            Console.WriteLine(-1);
        }
        else
        {
            Console.WriteLine(scores[x][y]);
        }
    }
}
