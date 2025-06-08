using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;

class Program
{
    static void Main(string[] args)
    {
        string s = Console.ReadLine();

        var lowersList = new List<string>();
        var sb = new StringBuilder();
        bool upperAppeared = false;
        foreach (char c in s)
        {
            sb.Append(c);
            if (Char.IsUpper(c))
            {
                if (upperAppeared)
                {
                    lowersList.Add(sb.ToString());
                    sb.Clear();
                    upperAppeared = false;
                }
                else
                {
                    upperAppeared = true;
                }
            }
        }

        var listSorted = lowersList.OrderBy(str => str.ToLower()).ToList();
        Console.WriteLine(string.Concat(listSorted));
    }
}
