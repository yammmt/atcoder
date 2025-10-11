using System;

class Program
{
    static void Main()
    {
        var nmk = Console.ReadLine().Split();
        var n = int.Parse(nmk[0]);
        var m = int.Parse(nmk[1]);
        var k = int.Parse(nmk[2]);
        var snm = new char[n][];
        for (int i = 0; i < n; i++)
        {
            snm[i] = Console.ReadLine().ToCharArray();
        }

        // マス目は高々 30x30 でしかない
        var ans = 0;
        for (int len = 1; len <= Math.Min(n, m); len++)
        {
            var curSize = len * len;
            for (int i = 0; i < n; i++)
            {
                if (i + len > n)
                {
                    break;
                }

                for (int j = 0; j < m; j++)
                {
                    if (j + len > m)
                    {
                        break;
                    }

                    var counter = new int[10];
                    for (int ii = i; ii < i + len; ii++)
                    {
                        for (int jj = j; jj < j + len; jj++)
                        {
                            counter[snm[ii][jj] - '0']++;
                        }
                    }

                    for (int ii = 0; ii < 10; ii++)
                    {
                        if (curSize - counter[ii] <= k)
                        {
                            // 更新判定が昇順であり Max 不要
                            ans = len;
                        }
                    }
                }
            }
        }

        Console.WriteLine(ans);
    }
}
