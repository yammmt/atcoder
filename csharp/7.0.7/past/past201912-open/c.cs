using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main(string[] args)
    {
        int[] nums = Console.ReadLine().Split(' ').Select(int.Parse).ToArray();
        Array.Sort(nums);
        Array.Reverse(nums);
        Console.WriteLine(nums[2]);
    }
}
