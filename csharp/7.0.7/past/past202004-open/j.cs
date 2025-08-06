using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main(string[] args)
    {
        char[] s = Console.ReadLine().ToCharArray();
        var n = (int)s.Length;

        // 愚直にやるだけで O(n^2) で間に合いそう, 最終出力が最大 1,000 字以下が保証されている
        // stack 用意して '(' の登場位置を管理して, ')' 出現時に一気に読み出す
        // 入力: ((ab)) みたいなのは見ておきたい, 出力は abbaabba
        // stack より list の方が高速そうだが
        var ans = new List<char>();
        var leftPos = new Stack<int>();

        for (int i = 0; i < n; i++)
        {
            if (s[i] == '(')
            {
                leftPos.Push(ans.Count);
            }
            else if (s[i] == ')')
            {
                var left = leftPos.Pop();
                var reversed = new List<char>();
                for (int j = left; j < ans.Count; j++)
                    reversed.Add(ans[j]);
                reversed.Reverse();
                for (int j = 0; j < reversed.Count; j++)
                    ans.Add(reversed[j]);
            }
            else
            {
                ans.Add(s[i]);
            }
        }

        Console.WriteLine(string.Join("", ans));
    }
}
