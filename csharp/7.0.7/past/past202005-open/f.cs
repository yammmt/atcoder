using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main()
    {
        const int ABC_NUM = 26;

        var n = int.Parse(Console.ReadLine());
        var ann = new char[n][];
        for (int i = 0; i < n; i++)
        {
            ann[i] = Console.ReadLine().ToCharArray();
        }

        var ans = new char[n / 2];

        for (int i = 0; i < n / 2; i++)
        {
            var appeared = new bool[ABC_NUM];
            for (int j = 0; j < n; j++)
            {
                var c = ann[i][j] - 'a';
                appeared[c] = true;
            }

            var passed = false;
            for (int j = 0; j < n; j++)
            {
                var c = ann[n - i - 1][j] - 'a';
                if (appeared[c])
                {
                    ans[i] = ann[n - i - 1][j];
                    passed = true;
                    break;
                }
            }

            if (!passed)
            {
                Console.WriteLine(-1);
                return;
            }
        }

        Console.Write(new string(ans));
        if (n % 2 == 1)
        {
            Console.Write(ann[n / 2][0]);
        }
        Array.Reverse(ans);
        Console.WriteLine(new string(ans));
    }
}
