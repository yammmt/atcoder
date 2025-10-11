using System;
using System.Collections.Generic;
using System.Runtime.InteropServices.Marshalling;

class Program
{
    static void Bfs(char[][] grid, bool[,] visited, (int, int) sij)
    {
        // (int, int)[] dij = { (-1, 0), (1, 0), (0, -1), (0, 1) };
        ReadOnlySpan<(int di, int dj)> dij = stackalloc (int, int)[]
        {
            (-1, 0), (1, 0), (0, -1), (0, 1)
        };

        var n = grid.Length;
        var m = grid[0].Length;

        var que = new Queue<(int, int)>();
        que.Enqueue(sij);
        while (que.TryDequeue(out var cur))
        {
            var (ci, cj) = cur;

            if (visited[ci, cj])
            {
                continue;
            }

            visited[ci, cj] = true;
            foreach (var (di, dj) in dij)
            {
                var ni = ci + di;
                var nj = cj + dj;
                if (ni < 0 || ni >= n || nj < 0 || nj >= m || grid[ni][nj] == '#')
                {
                    continue;
                }

                que.Enqueue((ni, nj));
            }
        }
    }

    static bool CouldVisitAll(char[][] grid)
    {
        var n = grid.Length;
        var m = grid[0].Length;

        var bfsRun = false;
        var visited = new bool[n, m];
        for (int i = 0; i < n; i++)
        {
            for (int j = 0; j < m; j++)
            {
                if (bfsRun)
                {
                    break;
                }

                if (grid[i][j] == '#')
                {
                    continue;
                }
                else
                {
                    Bfs(grid, visited, (i, j));
                    bfsRun = true;
                }
            }
        }

        // 入力には必ず壁でないマスがある
        for (int i = 0; i < n; i++)
        {
            for (int j = 0; j < m; j++)
            {
                if (grid[i][j] == '.' && !visited[i, j])
                {
                    return false;
                }
            }
        }

        return true;
    }

    static void Main()
    {
        var nm = Console.ReadLine().Split();
        var n = int.Parse(nm[0]);
        var m = int.Parse(nm[1]);
        var snm = new char[n][];
        for (int i = 0; i < n; i++)
        {
            snm[i] = Console.ReadLine().ToCharArray();
        }

        // 盤面が小さいので全探索
        int ans = 0;
        for (int i = 0; i < n; i++)
        {
            for (int j = 0; j < m; j++)
            {
                if (snm[i][j] == '.')
                {
                    continue;
                }

                // deep copy (clone)
                var scur = new char[n][];
                for (int k = 0; k < n; k++)
                {
                    scur[k] = (char[])snm[k].Clone();
                }

                scur[i][j] = '.';
                if (CouldVisitAll(scur))
                {
                    ans++;
                }
            }
        }

        Console.WriteLine(ans);
    }
}
