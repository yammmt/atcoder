using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main()
    {
        var nq = Console.ReadLine().Split();
        var n = int.Parse(nq[0]);
        var q = int.Parse(nq[1]);
        var an = Console.ReadLine().Split().Select(long.Parse).ToArray();

        for (int i = 0; i < q; i++)
        {
            var k = long.Parse(Console.ReadLine());

            if (k < an[0])
            {
                Console.WriteLine(k);
                continue;
            }

            // k より小さい自然数のうち pass 個が an[] の要素
            var pass = 0;
            var fail = n;
            while (fail - pass > 1)
            {
                var mid = (pass + fail) / 2;
                // an[mid]-mid-1: an[mid] より小さい自然数の中で, an[] に含まれていない個数
                // an[mid] より小さい自然数のうち, an[] に含まれている数は mid 個
                // -1: 3 より小さい数は 2 個, のような辻褄合わせ
                if (an[mid] - mid - 1 < k)
                {
                    pass = mid;
                }
                else
                {
                    fail = mid;
                }
            }

            Console.WriteLine((k + pass + 1));
        }
    }
}
