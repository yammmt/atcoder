using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main()
    {
        var n = int.Parse(Console.ReadLine());

        int ans = 0;
        for (int i = 0; i < n; i++)
        {
            ans += (n - i);
        }

        Console.WriteLine(ans);
    }
}