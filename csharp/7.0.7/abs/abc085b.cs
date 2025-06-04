using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main(string[] args)
    {
        int n = int.Parse(Console.ReadLine());
        int[] dn = new int[n];
        for (int i = 0; i < n; i++)
        {
            dn[i] = int.Parse(Console.ReadLine());
        }

        HashSet<int> hs = new HashSet<int>(dn);

        Console.WriteLine(hs.Count);
    }
}
