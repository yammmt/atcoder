using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main(string[] args)
    {
        var n = int.Parse(Console.ReadLine());
        var abn = Enumerable.Range(0, n)
            .Select(_ =>
            {
                var ab = Console.ReadLine().Split().Select(int.Parse).ToArray();
                return (a: ab[0] - 1, b: ab[1]);
            }).ToArray();
        Array.Sort(abn);

        // 昇順に出てくるので <価値, -価値> で雑にやる
        var pq = new PriorityQueue<int, int>();
        int ans = 0;
        int task_idx = 0;
        for (int i = 0; i < n; i++)
        {
            while (task_idx < n && abn[task_idx].a <= i)
            {
                pq.Enqueue(abn[task_idx].b, -abn[task_idx].b);
                task_idx++;
            }

            if (pq.TryDequeue(out int value, out int cost))
            {
                ans += value;
            }
            Console.WriteLine(ans);
        }
    }
}
