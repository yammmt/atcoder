using System;

class Program
{
    static void Main(string[] args)
    {
        string s = Console.ReadLine();
        int a = 0;
        int b = 0;
        int c = 0;
        foreach (char cc in s)
        {
            switch (cc)
            {
                case 'a':
                    a++;
                    break;
                case 'b':
                    b++;
                    break;
                case 'c':
                    c++;
                    break;
                default:
                    // unreachable
                    break;
            }
        }

        if (a > b && a > c)
        {
            Console.WriteLine("a");
        }
        else if (b > a && b > c)
        {
            Console.WriteLine("b");
        }
        else
        {
            Console.WriteLine("c");
        }
    }
}
