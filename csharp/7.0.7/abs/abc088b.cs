using System;
using System.Linq;

class Program
{
    static void Main(string[] args)
    {
        int n = int.Parse(Console.ReadLine());
        int[] an = Console.ReadLine().Split(' ').Select(int.Parse).ToArray();
        Array.Sort(an);
        Array.Reverse(an);

        int alice = 0;
        int bob = 0;
        for (int i = 0; i < n; i++)
        {
            if (i % 2 == 0)
            {
                alice += an[i];
            }
            else
            {
                bob += an[i];
            }
        }

        Console.WriteLine(alice - bob);
    }
}
