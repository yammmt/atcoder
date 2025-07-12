using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main()
    {
        const long MOD = 998_244_353;
        var s = Console.ReadLine();

        // 大きい数に対する剰余も, 剰余取って *10 しつつ次の項を足して剰余を取って…で求められる
        // 予め剰余を取っておけば, 掛け算は文字列でやらずともよい

        var nums = new Stack<long>();
        bool mulAppears = false;
        long numCur = 0;
        // 全部読んで掛け算を先にやる
        for (int i = 0; i < s.Length; i++)
        {
            if (s[i] == '+')
            {
                if (mulAppears)
                {
                    var a = nums.Pop();
                    nums.Push((a * numCur) % MOD);
                    mulAppears = false;
                }
                else
                {
                    nums.Push(numCur);
                }
                numCur = 0;
            }
            else if (s[i] == '*')
            {
                if (mulAppears)
                {
                    var a = nums.Pop();
                    nums.Push((a * numCur) % MOD);
                }
                else
                {
                    nums.Push(numCur);
                }
                numCur = 0;
                mulAppears = true;
            }
            else
            {
                var nCur = long.Parse(s[i].ToString());
                numCur = (numCur * 10 + nCur) % MOD;
            }
        }

        if (mulAppears)
        {
            var a = nums.Pop();
            nums.Push((a * numCur) % MOD);
        }
        else
        {
            nums.Push(numCur);
        }

        // 残ったものはすべて足すだけ
        long ans = 0;
        while (nums.TryPop(out var a))
        {
            // Console.WriteLine(a);
            ans = (ans + a) % MOD;
        }

        Console.WriteLine(ans);
    }
}