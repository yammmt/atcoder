using System;
class Program
{
    static void Main(string[] args)
    {
        string s = Console.ReadLine();

        int ans = 0;
        for (int i = 0; i < 3; i++)
        {
            if (s[i] == '1')
            {
                ans++;
            }
        }
        Console.WriteLine(ans);
    }
}
