using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main()
    {
        string x = Console.ReadLine();
        var zeros = 0;
        foreach (var xx in x)
            if (xx == '0')
                zeros++;

        var ans = x.Length;
        if (x[0] == '1' && zeros == x.Length - 1)
            ans--;

        Console.WriteLine(ans);
    }
}