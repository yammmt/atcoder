using System;
using System.Linq;

class Program
{
    static void Main(string[] args)
    {
        int[] s = Console.ReadLine().Split(' ').Select(int.Parse).ToArray();
        int n = s[0];
        int y = s[1];

        for (int i = 0; i <= n; i++)
        {
            int a = 10000 * i;
            if (a > y)
            {
                break;
            }

            for (int j = 0; j <= n - i; j++)
            {
                int b = 5000 * j;
                if (a + b > y)
                {
                    break;
                }

                int c = 1000 * (n - i - j);
                if (a + b + c == y)
                {
                    Console.WriteLine(i + " " + j + " " + c / 1000);
                    return;
                }
            }
        }

        Console.WriteLine("-1 -1 -1");
    }
}
