using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Swap(ref long a, ref long b)
    {
        (a, b) = (b, a);
    }

    static void Main()
    {
        var n = long.Parse(Console.ReadLine());
        var q = int.Parse(Console.ReadLine());

        var rows = new long[n];
        var columns = new long[n];
        for (int i = 0; i < n; i++)
        {
            rows[i] = i;
            columns[i] = i;
        }
        bool isTransposed = false;

        for (int i = 0; i < q; i++)
        {
            var query = Console.ReadLine().Split(' ').Select(int.Parse).ToArray();
            switch (query[0])
            {
                case 1:
                    // 行の交換
                    var qa1 = query[1] - 1;
                    var qb1 = query[2] - 1;
                    if (isTransposed)
                    {
                        Swap(ref columns[qa1], ref columns[qb1]);
                    }
                    else
                    {
                        Swap(ref rows[qa1], ref rows[qb1]);
                    }
                    break;
                case 2:
                    // 列の交換
                    var qa2 = query[1] - 1;
                    var qb2 = query[2] - 1;
                    if (isTransposed)
                    {
                        Swap(ref rows[qa2], ref rows[qb2]);
                    }
                    else
                    {
                        Swap(ref columns[qa2], ref columns[qb2]);
                    }
                    break;
                case 3:
                    isTransposed = !isTransposed;
                    break;
                case 4:
                    var qa4 = query[1] - 1;
                    var qb4 = query[2] - 1;
                    var iRow = isTransposed ? qb4 : qa4;
                    var iCol = isTransposed ? qa4 : qb4;
                    Console.WriteLine(n * rows[iRow] + columns[iCol]);
                    break;
                default:
                    break;
            }
        }
    }
}
