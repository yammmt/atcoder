using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main(string[] args)
    {
        const int Unvisited = 1_000_000_000;

        var nm = Console.ReadLine().Split().Select(int.Parse).ToArray();
        var n = nm[0];
        var m = nm[1];
        var anm = new int[n][];
        var levels = Enumerable.Range(0, 11).Select(_ => new List<(int x, int y)>()).ToList();
        for (int i = 0; i < n; i++)
        {
            var line = Console.ReadLine();
            var row = new int[m];
            for (int j = 0; j < m; j++)
            {
                row[j] = line[j] switch
                {
                    'S' => 0,
                    'G' => 10,
                    var ch => ch - '0'
                };
                levels[row[j]].Add((i, j));
            }
            anm[i] = row;
        }

        // マス間の移動距離は O(1) で求められる

        var costs = InitCost(n, m, Unvisited);
        var (firstx, firsty) = levels[0].First();
        costs[firstx, firsty] = 0;
        for (int i = 1; i <= 10; i++)
        {
            var costs_cur = InitCost(n, m, Unvisited);
            foreach (var (sx, sy) in levels[i - 1])
            {
                foreach (var (gx, gy) in levels[i])
                {
                    costs_cur[gx, gy] = Math.Min(
                        costs_cur[gx, gy],
                        costs[sx, sy] + Math.Abs(sx - gx) + Math.Abs(sy - gy)
                    );
                }
            }

            costs = costs_cur;
        }

        var (lastx, lasty) = levels[10].First();
        Console.WriteLine(
            (costs[lastx, lasty] == Unvisited) ? "-1" : costs[lastx, lasty].ToString()
        );
    }

    static int[,] InitCost(int n, int m, int value)
    {
        var cost = new int[n, m];
        for (int i = 0; i < n; i++)
        {
            for (int j = 0; j < m; j++)
            {
                cost[i, j] = value;
            }
        }
        return cost;
    }
}
