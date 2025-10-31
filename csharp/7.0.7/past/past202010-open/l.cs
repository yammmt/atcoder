using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main()
    {
        var nq = Console.ReadLine().Split();
        var n = int.Parse(nq[0]);
        var q = int.Parse(nq[1]);
        var hn = Console.ReadLine().Split().Select(long.Parse).ToArray();

        // それぞれの, が厄介
        // 最後にまとめて判定であればやるだけ問題
        // 偶数の操作は奇数のマイナス操作と置き換えてよい
        // 隣り合うマンションの高さの差を i の偶奇で分けて管理して,
        // 差を読み替えながらうまく進める

        // 問題文と合わせて 1-origin
        var odd = new Dictionary<long, int>();
        for (int i = 0; i < n - 1; i += 2)
        {
            var v = hn[i + 1] - hn[i];
            if (odd.TryGetValue(v, out var c))
                odd[v] = c + 1;
            else
                odd.Add(v, 1);
        }
        var even = new Dictionary<long, int>();
        for (int i = 1; i < n - 1; i += 2)
        {
            var v = hn[i + 1] - hn[i];
            if (even.TryGetValue(v, out var c))
                even[v] = c + 1;
            else
                even.Add(v, 1);
        }

        long oddAdded = 0;
        long evenAdded = 0;
        for (int i = 0; i < q; i++)
        {
            var qq = Console.ReadLine().Split();
            var t = int.Parse(qq[0]);
            long v;
            switch (t)
            {
                case 1:
                    v = long.Parse(qq[1]);
                    oddAdded += v;
                    break;
                case 2:
                    v = long.Parse(qq[1]);
                    evenAdded += v;
                    break;
                case 3:
                    var u = int.Parse(qq[1]) - 1;
                    v = long.Parse(qq[2]);
                    if (u % 2 == 0)
                    {
                        // 奇数項をいじる
                        if (u + 1 < n)
                        {
                            var oddBefore = hn[u + 1] - hn[u];
                            odd[oddBefore]--;
                            var oddAfter = hn[u + 1] - (hn[u] + v);
                            if (odd.TryGetValue(oddAfter, out var c))
                                odd[oddAfter] = c + 1;
                            else
                                odd.Add(oddAfter, 1);
                        }
                        // 偶数項をいじる
                        if (u - 1 >= 0)
                        {
                            var evenBefore = hn[u] - hn[u - 1];
                            even[evenBefore]--;
                            var evenAfter = hn[u] + v - hn[u - 1];
                            if (even.TryGetValue(evenAfter, out var c))
                                even[evenAfter] = c + 1;
                            else
                                even.Add(evenAfter, 1);
                        }
                    }
                    else
                    {
                        // 奇数項をいじる
                        if (u + 1 < n)
                        {
                            var evenBefore = hn[u + 1] - hn[u];
                            even[evenBefore]--;
                            var evenAfter = hn[u + 1] - (hn[u] + v);
                            if (even.TryGetValue(evenAfter, out var c))
                                even[evenAfter] = c + 1;
                            else
                                even.Add(evenAfter, 1);
                        }
                        // 偶数項をいじる
                        if (u - 1 >= 0)
                        {
                            var oddBefore = hn[u] - hn[u - 1];
                            odd[oddBefore]--;
                            var oddAfter = hn[u] + v - hn[u - 1];
                            if (odd.TryGetValue(oddAfter, out var c))
                                odd[oddAfter] = c + 1;
                            else
                                odd.Add(oddAfter, 1);
                        }
                    }

                    hn[u] += v;
                    break;
                default:
                    break;
            }

            int ans = 0;
            long oMinusE = oddAdded - evenAdded;
            if (odd.TryGetValue(oMinusE, out var v11))
                ans += v11;
            if (even.TryGetValue(-oMinusE, out var v12))
                ans += v12;
            Console.WriteLine($"{ans}");
        }
    }
}
