using System;
using System.Linq;

class Program
{
    static bool IsValidAns(char[] t, char[] s)
    {
        if (t.SequenceEqual(s))
            return false;

        if (t.Reverse().SequenceEqual(s))
            return false;

        // 入れ替え部分は省略

        return true;
    }

    static char[] Dfs(int curPos, int[] orders, bool[] isAvailable, char[] s)
    {
        var n = orders.Length;

        if (curPos == n)
        {
            var t = new char[n];
            for (int j = 0; j < n; j++)
            {
                t[j] = s[orders[j]];
            }
            if (IsValidAns(t, s))
            {
                return t;
            }
            else
            {
                return null;
            }
        }

        for (int i = 0; i < n; i++)
        {
            if (!isAvailable[i])
                continue;

            orders[curPos] = i;
            isAvailable[i] = false;

            var res = Dfs(curPos + 1, orders, isAvailable, s);
            if (res != null)
                return res;

            // orders は判定なしに更新されるので放置でよい
            isAvailable[i] = true;
        }

        return null;
    }

    static void Main()
    {
        var n = int.Parse(Console.ReadLine());
        var s = Console.ReadLine().ToCharArray();

        int[] orders = Enumerable.Repeat(-1, n).ToArray();
        bool[] isAvailable = Enumerable.Repeat(true, n).ToArray();
        char[] ans = Dfs(0, orders, isAvailable, s);

        if (ans == null)
        {
            Console.WriteLine("None");
        }
        else
        {
            Console.WriteLine(ans);
        }
    }
}
