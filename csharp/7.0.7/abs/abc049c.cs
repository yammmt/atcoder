using System;
using System.Linq;

class Program
{
    static void Main(string[] args)
    {
        char[] s = Console.ReadLine().ToCharArray();
        Array.Reverse(s);
        int i = 0;
        while (i < s.Length)
        {
            // Console.WriteLine(i);
            if (i + 6 < s.Length && new string(s, i, 7) == "remaerd")
            {
                // dreamer
                i += 7;
            }
            else if (i + 4 < s.Length && new string(s, i, 5) == "maerd")
            {
                // dream
                i += 5;
            }
            else if (i + 5 < s.Length && new string(s, i, 6) == "resare")
            {
                // eraser
                i += 6;
            }
            else if (i + 4 < s.Length && new string(s, i, 5) == "esare")
            {
                // erase
                i += 5;
            }
            else
            {
                Console.WriteLine("NO");
                return;
            }
        }

        Console.WriteLine("YES");
    }
}
