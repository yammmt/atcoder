using System;
using System.Collections.Generic;

class Program
{
    static void Main(string[] args)
    {
        int n = int.Parse(Console.ReadLine());
        List<(int t, int x, int y)> txyn = new List<(int t, int x, int y)>();
        for (int i = 0; i < n; i++)
        {
            string[] s = Console.ReadLine().Split();
            txyn.Add((int.Parse(s[0]), int.Parse(s[1]), int.Parse(s[2])));
        }

        var cur = (t: 0, x: 0, y: 0);
        foreach (var next in txyn)
        {
            int t_diff = next.t - cur.t;
            int dist = Math.Abs(next.x - cur.x) + Math.Abs(next.y - cur.y);
            if (t_diff < dist || (t_diff - dist) % 2 != 0)
            {
                Console.WriteLine("No");
                return;
            }

            cur = next;
        }

        Console.WriteLine("Yes");
    }
}
