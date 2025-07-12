using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main()
    {
        var nmk = Console.ReadLine().Split().Select(long.Parse).ToArray();
        var n = nmk[0];
        var m = nmk[1];
        var k = nmk[2];
        var an = Console.ReadLine().Split().Select(long.Parse).ToArray();

        // 全員の飴の数を pass 個以上にできるか？
        long pass = 0;
        long fail = an.Max() + m * k + 1;
        while (fail - pass > 1)
        {
            long mid = (pass + fail) / 2;

            long mCur = 0;
            foreach (var a in an)
            {
                if (mid > a)
                {
                    var muled = (mid - a + k - 1) / k;
                    mCur += muled;
                }
            }

            if (mCur <= m)
            {
                pass = mid;
            }
            else
            {
                fail = mid;
            }
        }

        var ansMin = new long[n];
        long mAns = 0;
        for (int i = 0; i < n; i++)
        {
            if (pass > an[i])
            {
                var muled = (pass - an[i] + k - 1) / k;
                ansMin[i] = an[i] + k * muled;
                mAns += muled;
            }
            else
            {
                ansMin[i] = an[i];
            }
        }
        // m 回の動作終了後に端数を足す
        // <(idx, 飴数), 飴数>
        var pq = new PriorityQueue<(int, long), (long, int)>();
        for (int i = 0; i < n; i++)
            pq.Enqueue((i, ansMin[i]), (ansMin[i], i));
        for (long i = 0; i < m - mAns; i++)
        {
            var (pi, pc) = pq.Dequeue();
            pq.Enqueue((pi, pc + k), (pc + k, pi));
        }

        var ans = new long[n];
        while (pq.TryDequeue(out var a, out var b))
        {
            var (i, c) = a;
            ans[i] = c;
        }

        Console.WriteLine(string.Join(" ", ans));
    }
}