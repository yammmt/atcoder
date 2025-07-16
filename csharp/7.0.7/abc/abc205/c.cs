using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main()
    {
        var abc = Console.ReadLine().Split().Select(int.Parse).ToArray();
        var a = abc[0];
        var b = abc[1];
        var c = abc[2];

        if (c % 2 == 0)
        {
            if (Math.Abs(a) > Math.Abs(b))
            {
                Console.WriteLine(">");
            }
            else if (Math.Abs(a) < Math.Abs(b))
            {
                Console.WriteLine("<");
            }
            else
            {
                Console.WriteLine("=");
            }
        }
        else
        {
            if (a > b)
            {
                Console.WriteLine(">");
            }
            else if (a < b)
            {
                Console.WriteLine("<");
            }
            else
            {
                Console.WriteLine("=");
            }
        }
    }
}