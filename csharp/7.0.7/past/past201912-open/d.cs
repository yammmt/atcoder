using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main(string[] args)
    {
        int n = int.Parse(Console.ReadLine());
        int[] nums = new int[n + 1];
        for (int i = 0; i < n; i++)
        {
            int a = int.Parse(Console.ReadLine());
            nums[a]++;
        }

        int removed = 0;
        int added = 0;
        for (int i = 1; i < n + 1; i++)
        {
            if (nums[i] == 0)
            {
                removed = i;
            }
            else if (nums[i] == 2)
            {
                added = i;
            }
        }

        if (removed == 0)
        {
            Console.WriteLine("Correct");
        }
        else
        {
            Console.WriteLine(added + " " + removed);
        }
    }
}
