using System;
using System.Linq;

class Program
{
    static void Main()
    {
        var nm = Console.ReadLine().Split();
        var n = int.Parse(nm[0]);
        var m = int.Parse(nm[1]);

        var abcm = new (int A, int B, int C)[m];
        for (int i = 0; i < m; i++)
        {
            var x = Console.ReadLine().Split().Select(s => int.Parse(s) - 1).ToArray();
            abcm[i] = (x[0], x[1], x[2]);
        }

        // 2^14 は全探索できる
        var ans = 0;
        for (int i = 0; i < (1 << n); i++)
        {
            var isSelected = new bool[n];
            var ii = i;
            for (int j = 0; j < n; j++)
            {
                if (ii % 2 == 1)
                    isSelected[j] = true;

                ii /= 2;
            }

            var passed = true;
            foreach (var (a, b, c) in abcm)
            {
                if (isSelected[a] && isSelected[b] && isSelected[c])
                {
                    passed = false;
                    break;
                }
            }
            // 既に爆発
            if (!passed)
                continue;

            var score = 0;
            for (int j = 0; j < n; j++)
            {
                if (isSelected[j])
                    continue;

                var isSelectedCur = isSelected.Clone() as bool[];
                isSelectedCur[j] = true;
                foreach (var (a, b, c) in abcm)
                    if (isSelectedCur[a] && isSelectedCur[b] && isSelectedCur[c])
                    {
                        score++;
                        break;
                    }
            }
            ans = Math.Max(ans, score);
        }

        Console.WriteLine(ans);
    }
}
