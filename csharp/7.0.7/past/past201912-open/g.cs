using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main(string[] args)
    {
        int n = int.Parse(Console.ReadLine());
        // 嫌な入力形式
        int[,] ann = new int[n, n];
        for (int i = 0; i < n - 1; i++)
        {
            int[] arr = Console.ReadLine().Split(' ').Select(int.Parse).ToArray();
            for (int j = 0; j < arr.Length; j++)
            {
                ann[i, j + i + 1] = arr[j];
            }
        }

        int ans = -1_000_000 * 10;
        for (int i = 0; i < (int)Math.Pow(3, n); i++)
        {
            int bit = i;
            var groups = new List<List<int>>();
            for (int j = 0; j < 3; j++)
            {
                groups.Add(new List<int>());
            }

            for (int j = 0; j < n; j++)
            {
                groups[bit % 3].Add(j);
                bit /= 3;
            }

            int cur = 0;
            for (int j = 0; j < 3; j++)
            {
                for (int k = 0; k < groups[j].Count; k++)
                {
                    for (int l = k + 1; l < groups[j].Count; l++)
                    {
                        cur += ann[groups[j][k], groups[j][l]];
                    }
                }
            }

            ans = Math.Max(ans, cur);
        }

        Console.WriteLine(ans);
    }
}
