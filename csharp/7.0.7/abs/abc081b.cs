using System;
using System.Linq;

class Program
{
    static void Main(string[] args)
    {
        // 整数の入力
        int _n = int.Parse(Console.ReadLine());
        // スペース区切りの整数の入力
        int[] an = Console.ReadLine().Split(' ').Select(int.Parse).ToArray();

        int ans = 1_000_000_000;
        foreach (int a in an)
        {
            int cur = 0;
            int aa = a;
            while (aa % 2 == 0)
            {
                cur++;
                aa /= 2;
            }
            ans = Math.Min(ans, cur);
        }

        Console.WriteLine(ans);
    }
}
