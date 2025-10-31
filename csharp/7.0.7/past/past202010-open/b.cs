using System;

class Program
{
    static void Main()
    {
        var xy = Console.ReadLine().Split();
        var x = int.Parse(xy[0]);
        var y = int.Parse(xy[1]);

        if (y == 0)
        {
            Console.WriteLine("ERROR");
            return;
        }

        Console.Write($"{x / y}.");
        for (int i = 0; i < 2; i++)
        {
            x %= y;
            x *= 10;
            Console.Write(x/y);
        }
        Console.WriteLine();
    }
}
