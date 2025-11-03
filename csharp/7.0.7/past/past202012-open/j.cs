using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main()
    {
        const long XMAX = 1_000_000_000_000_001;

        var s = Console.ReadLine().ToCharArray();
        var n = s.Length;
        var x = long.Parse(Console.ReadLine());

        // 前から順に出力文字数を数えていき, 合計が X 以上になった際に
        // 直前までの連結文字列から再帰的に求める
        // 愚直に直前の出力文字数をもつ方針では, 出力文字数合計が大きくなりすぎて TLE

        // その字を出力した時点での出力文字数合計
        var charSum = new long[n];
        var cnt = 0L;
        for (int i = 0; i < n; i++)
        {
            if (Char.IsNumber(s[i]))
            {
                var c = s[i] - '0';
                cnt = Math.Min(cnt * (c + 1), XMAX);
                charSum[i] = cnt;
            }
            else
            {
                cnt = Math.Min(cnt + 1, XMAX);
                charSum[i] = cnt;
            }
        }

        var lastI = 0;
        var curX = x;
        for (int i = 0; i < n; i++)
        {
            if (charSum[i] >= x)
            {
                lastI = i;
                break;
            }
        }

        for (int i = lastI; i >= 0; i--)
        {
            long prev = i > 0 ? charSum[i - 1] : 0;
            if (char.IsNumber(s[i]))
            {
                // 数字の場合には答えとなり得ない
                // 同じ文字列の繰り返しであるので, 剰余を使ってよい
                curX = (curX - 1) % prev + 1;
            }
            else if (curX == charSum[i])
            {
                Console.WriteLine(s[i]);
                return;
            }
        }
    }
}
