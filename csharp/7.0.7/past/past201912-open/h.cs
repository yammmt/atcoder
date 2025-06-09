using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main(string[] args)
    {
        int n = int.Parse(Console.ReadLine());
        long[] cn = Console.ReadLine().Split(' ').Select(long.Parse).ToArray();
        int q = int.Parse(Console.ReadLine());

        // origin 違うので問題文と偶数奇数が逆

        // (idx, 在庫数)
        (int, long) evenMin = (0, 1_000_000_002);
        (int, long) oddMin = (0, 1_000_000_002);
        long[] soldOne = new long[n];
        long soldEven = 0;
        long soldAll = 0;

        for (int i = 0; i < n; i++)
        {
            if (i % 2 == 0)
            {
                if (cn[i] < evenMin.Item2)
                {
                    evenMin.Item1 = i;
                    evenMin.Item2 = cn[i];
                }
            }
            else
            {
                if (cn[i] < oddMin.Item2)
                {
                    oddMin.Item1 = i;
                    oddMin.Item2 = cn[i];
                }
            }
        }

        long ans = 0;

        for (int i = 0; i < q; i++)
        {
            string[] s = Console.ReadLine().Split(' ');
            switch (int.Parse(s[0]))
            {
                case 1:
                    {
                        // x を a 枚販売する
                        int x = int.Parse(s[1]) - 1;
                        long a = long.Parse(s[2]);
                        long sold = soldOne[x] + soldAll;
                        if (x % 2 == 0)
                        {
                            sold += soldEven;
                        }
                        if (sold + a <= cn[x])
                        {
                            ans += a;
                            soldOne[x] += a;

                            if (x % 2 == 0 && cn[x] - soldOne[x] < evenMin.Item2)
                            {
                                evenMin = (x, cn[x] - soldOne[x]);
                            }
                            else if (x % 2 == 1 && cn[x] - soldOne[x] < oddMin.Item2)
                            {
                                oddMin = (x, cn[x] - soldOne[x]);
                            }
                        }
                        break;
                    }
                case 2:
                    {
                        // 偶数カードを a 枚ずつ販売する
                        long a = long.Parse(s[1]);
                        if (evenMin.Item2 - soldEven - soldAll - a < 0)
                        {
                            // 販売できない
                            break;
                        }

                        soldEven += a;
                        // 端数切り上げ
                        ans += a * ((n + 1) / 2);
                        break;
                    }
                case 3:
                    {
                        // 全種類 a 枚ずつ販売する
                        long a = long.Parse(s[1]);
                        if (
                            evenMin.Item2 - soldEven - soldAll - a < 0
                            || oddMin.Item2 - soldAll - a < 0
                        )
                        {
                            break;
                        }

                        soldAll += a;
                        ans += a * n;
                        break;
                    }
                default:
                    // unreachable
                    break;
            }
            // Console.WriteLine("i: " + i + ", ans: " + ans);
        }

        Console.WriteLine(ans);
    }
}
