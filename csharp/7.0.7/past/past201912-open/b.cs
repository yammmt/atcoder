class Program
{
    static void Main(string[] args)
    {
        int n = int.Parse(Console.ReadLine());
        int[] an = new int[n];
        for (int i = 0; i < n; i++)
        {
            int a = int.Parse(Console.ReadLine());
            an[i] = a;
        }

        int a_cur = an[0];
        for (int i = 1; i < n; i++)
        {
            if (an[i] == a_cur)
            {
                Console.WriteLine("stay");
            }
            else if (an[i] > a_cur)
            {
                Console.WriteLine("up " + (an[i] - a_cur));
            }
            else
            {
                Console.WriteLine("down " + (a_cur - an[i]));
            }
            a_cur = an[i];
        }
    }
}
