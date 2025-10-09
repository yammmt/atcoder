using System;
using System.Linq;

class Program
{
    static void Main()
    {
        var n = int.Parse(Console.ReadLine());
        var s = Console.ReadLine().ToCharArray();

        // 左右端に向かう分の x + y は初手で必要としてよいとして, と考えずとも
        // N が高々 50 であるために全探索してしまってよい
        // 左に [0, 50] 回, 右に [0, 50] 回

        var ans_xy = Int32.MaxValue;
        var ans_x = Int32.MaxValue;
        var ans_y = Int32.MaxValue;
        for (var x = 0; x <= n; x++)
        {
            for (var y = 0; y <= n; y++)
            {
                var xy = x + y;
                if (xy >= ans_xy)
                {
                    break;
                }

                var ss = (char[])s.Clone();
                for (var i = 0; i < n; i++)
                {
                    if (s[i] == '#')
                    {
                        // to left
                        for (var j = 1; j <= x; j++)
                        {
                            var k = i - j;
                            if (k < 0)
                            {
                                break;
                            }

                            ss[k] = '#';
                        }

                        // to right
                        for (var j = 1; j <= y; j++)
                        {
                            var k = i + j;
                            if (k >= n)
                            {
                                break;
                            }

                            ss[k] = '#';
                        }
                    }
                }

                if (ss.All(c => c == '#') && xy < ans_xy)
                {
                    ans_xy = xy;
                    ans_x = x;
                    ans_y = y;
                }
            }
        }

        Console.WriteLine(ans_x + " " + ans_y);
    }
}
