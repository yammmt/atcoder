using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main()
    {
        // k は最大で 60C30 ≒ 1.18e17
        var abk = Console.ReadLine().Split().Select(long.Parse).ToArray();
        var a = abk[0];
        var b = abk[1];
        var k = abk[2] - 1;

        // 先頭の x 個を a として作れる文字列の種類は (A+B-x-1)C(A-x) 個,
        // -1 は後続文字列の先頭を b とするための項
        // 先頭 x 個で作れる数が K 以下かつ先頭 x+1 個で作れる数が K より大きいとなれば
        // 先頭 x 個は a でよいと確定させられる
        // 次は残った A-x 個の a と…として, 一つ一つ桁を確定させていく
        // 実装めんどい感じがする…というか一文字ずつ見ればよい, これだと -1 もいらない

        // nCr
        int size = (int)(a + b + 2);
        var ncr = new long[size, size];
        ncr[0, 0] = 1;
        for (int i = 0; i < size - 1; i++)
        {
            for (int j = 0; j <= i; j++)
            {
                ncr[i + 1, j] += ncr[i, j];
                ncr[i + 1, j + 1] += ncr[i, j];
            }
        }

        var ans = new char[a + b];
        var aLeft = a;
        var bLeft = b;
        for (int i = 0; i < a + b; i++)
        {
            // いる？
            if (aLeft == 0)
            {
                ans[i] = 'b';
                bLeft--;
                continue;
            }
            else if (bLeft == 0)
            {
                ans[i] = 'a';
                aLeft--;
                continue;
            }

            // 今の項を a として, 残りで作れる文字列数が
            //   - k より大きい => 'a', 求める文字列より小さいものを k 個にできる
            //   - k 以下 => 'b', 求める文字列より小さいものは最大でも k-1 個になってしまう
            //              かつ, b 確定により自身より小さいことが確定した文字列数分を差し引く
            var combinationsLeft = ncr[aLeft + bLeft - 1, aLeft - 1];
            if (combinationsLeft > k)
            {
                ans[i] = 'a';
                aLeft--;
            }
            else
            {
                ans[i] = 'b';
                bLeft--;
                k -= combinationsLeft;
            }
        }

        Console.WriteLine(new string(ans));
    }
}
