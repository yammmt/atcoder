using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main(string[] args)
    {
        int[] s = Console.ReadLine().Split(' ').Select(int.Parse).ToArray();
        int n = s[0];
        int q = s[1];
        bool[,] x_follows_y = new bool[n, n];

        for (int i = 0; i < q; i++)
        {
            int[] ss = Console.ReadLine().Split(' ').Select(int.Parse).ToArray();
            int a = ss[1] - 1;
            if (ss[0] == 1)
            {
                // フォロー
                int b = ss[2] - 1;
                x_follows_y[a, b] = true;
            }
            else if (ss[0] == 2)
            {
                // フォロー全返し
                for (int j = 0; j < n; j++)
                {
                    if (x_follows_y[j, a])
                    {
                        x_follows_y[a, j] = true;
                    }
                }
            }
            else
            {
                // フォローフォロー
                bool[] will_follow = new bool[n];
                for (int j = 0; j < n; j++)
                {
                    if (x_follows_y[a, j])
                    {
                        for (int k = 0; k < n; k++)
                        {
                            if (k == a)
                            {
                                continue;
                            }

                            if (x_follows_y[j, k])
                            {
                                will_follow[k] = true;
                            }
                        }
                    }
                }
                for (int j = 0; j < n; j++)
                {
                    if (will_follow[j]) {
                        x_follows_y[a, j] = true;
                    }
                }
            }
        }

        for (int i = 0; i < n; i++)
        {
            for (int j = 0; j < n; j++)
            {
                Console.Write(x_follows_y[i, j] ? "Y" : "N");
            }
            Console.WriteLine();
        }
    }
}
