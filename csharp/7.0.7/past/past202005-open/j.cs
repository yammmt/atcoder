using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main()
    {
        var nm = Console.ReadLine().Split(' ');
        var n = int.Parse(nm[0]);
        var m = int.Parse(nm[1]);
        var am = Console.ReadLine().Split(' ').Select(int.Parse).ToArray();

        // 子供が属した美味しさは降順に並ぶ, ので二分探索
        // am[] の値より小さい最小/左の数を選ぶ
        var deliciousness = new int[n];
        foreach (int a in am)
        {
            var pass = n;
            var fail = -1;
            while (pass - fail > 1)
            {
                var mid = (pass + fail) / 2;
                if (deliciousness[mid] < a)
                {
                    pass = mid;
                }
                else
                {
                    fail = mid;
                }
            }

            if (pass == n)
            {
                Console.WriteLine(-1);
            }
            else
            {
                Console.WriteLine(pass + 1);
                deliciousness[pass] = a;
            }
        }
    }
}
