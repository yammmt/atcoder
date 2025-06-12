using System;

class Program
{
    static bool IsMatch(char a, char b)
    {
        if (a == '.' || a == b)
        {
            return true;
        }
        else
        {
            return false;
        }
    }

    static void Main()
    {
        string abc = "abcdefghijklmnopqrstuvwxyz.";

        // 問題文が難しい..
        string s = Console.ReadLine();

        int ans = 0;

        // 一文字
        for (int i = 0; i < abc.Length; i++)
        {
            for (int j = 0; j < s.Length; j++)
            {
                if (IsMatch(abc[i], s[j]))
                {
                    ans++;
                    break;
                }
            }
        }

        // 二文字
        for (int i = 0; i < abc.Length; i++)
        {
            for (int j = 0; j < abc.Length; j++)
            {
                for (int k = 0; k < s.Length - 1; k++)
                {
                    if (IsMatch(abc[i], s[k]) && IsMatch(abc[j], s[k + 1]))
                    {
                        ans++;
                        break;
                    }
                }
            }
        }

        // 三文字
        for (int i = 0; i < abc.Length; i++)
        {
            for (int j = 0; j < abc.Length; j++)
            {
                for (int k = 0; k < abc.Length; k++)
                {
                    for (int l = 0; l < s.Length - 2; l++)
                    {
                        if (IsMatch(abc[i], s[l]) && IsMatch(abc[j], s[l + 1]) && IsMatch(abc[k], s[l + 2]))
                        {
                            ans++;
                            break;
                        }
                    }
                }
            }
        }

        Console.WriteLine(ans);
    }
}
