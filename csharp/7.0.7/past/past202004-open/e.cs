using System;
using System.Linq;

class Program
{
    static void Main(string[] args)
    {
        // 問題文が読み難い
        var n = int.Parse(Console.ReadLine());
        var an = Console.ReadLine().Split().Select(int.Parse).ToArray();
        for (int i = 0; i < n; i++) an[i]--;

        for (int i = 0; i < n; i++)
        {
            int x = an[i];
            int ans = 1;
            while (x != i)
            {
                x = an[x];
                ans++;
            }
            Console.Write(ans);
            if (i == n - 1)
            {
                Console.WriteLine();
            }
            else
            {
                Console.Write(" ");
            }
        }
    }
}
