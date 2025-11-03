using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static (int, int)[] DIRS = new (int, int)[] { (-1, 0), (1, 0), (0, -1), (0, 1) };

    static bool Bfs((int, int) curPos, HashSet<(int, int)> unvisited, char[][] shw)
    {
        unvisited.Remove(curPos);
        if (unvisited.Count == 0)
        {
            Console.WriteLine($"{curPos.Item1 + 1} {curPos.Item2 + 1}");
            return true;
        }

        var h = shw.Length;
        var w = shw[0].Length;
        foreach (var dir in DIRS)
        {
            var xNext = curPos.Item1 + dir.Item1;
            var yNext = curPos.Item2 + dir.Item2;
            if (xNext < 0 || xNext >= h || yNext < 0 || yNext >= w || !unvisited.Contains((xNext, yNext)))
                continue;

            if (Bfs((xNext, yNext), unvisited, shw))
            {
                Console.WriteLine($"{curPos.Item1 + 1} {curPos.Item2 + 1}");
                return true;
            }
        }
        unvisited.Add((curPos));

        return false;
    }

    static void Main()
    {
        var hw = Console.ReadLine().Split();
        var h = int.Parse(hw[0]);
        var w = int.Parse(hw[1]);
        var shw = new char[h][];
        for (int i = 0; i < h; i++)
            shw[i] = Console.ReadLine().ToCharArray();

        // 問題文: # を一筆書きする順で出力しろ, っぽい
        // 点数は高々 16 点だが, 16! は間に合わない
        // 左上を始点決め打ちでもうまくいかない場合がある (例 2)
        // 再帰 BFS で全探索で時間間に合う？はず

        var unvisited = new HashSet<(int, int)>();
        for (int i = 0; i < h; i++)
            for (int j = 0; j < w; j++)
            {
                if (shw[i][j] == '#')
                    unvisited.Add((i, j));
            }

        Console.WriteLine(unvisited.Count);
        for (int i = 0; i < h; i++)
            for (int j = 0; j < w; j++)
            {
                if (shw[i][j] != '#')
                    continue;

                // clone
                var hs = new HashSet<(int, int)>(unvisited);
                if (Bfs((i, j), hs, shw))
                    return;
            }
    }
}
