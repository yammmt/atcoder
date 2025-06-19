using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main()
    {
        var s = Console.ReadLine();

        int ans = 0;
        for (int i = 0; i < 10000; i++)
        {
            var contains = new bool[10];
            var j = i;
            for (int k = 0; k < 4; k++)
            {
                contains[j % 10] = true;
                j /= 10;
            }

            var passed = true;
            for (int k = 0; k < 10; k++)
            {
                if ((s[k] == 'o' && !contains[k]) || (s[k] == 'x' && contains[k]))
                {
                    passed = false;
                    break;
                }
            }

            if (passed) ans++;
        }

        Console.WriteLine(ans);
    }
}