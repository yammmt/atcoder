using System;

class Program
{
    static void Main(string[] args)
    {
        string[] ss = Console.ReadLine().Split(' ');
        string s = ss[0];
        string t = ss[1];

        string[] floors = new string[18];
        int i = 0;
        for (int j = 9; j > 0; j--)
        {
            floors[i++] = "B" + j.ToString();
        }
        for (int j = 1; j < 10; j++)
        {
            floors[i++] = j.ToString() + "F";
        }

        Console.WriteLine(Math.Abs(Array.IndexOf(floors, s) - Array.IndexOf(floors, t)));
    }
}
