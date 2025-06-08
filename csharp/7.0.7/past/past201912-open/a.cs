class Program
{
    static void Main(string[] args)
    {
        if (int.TryParse(Console.ReadLine(), out int v))
        {
            Console.WriteLine(v * 2);
        }
        else
        {
            Console.WriteLine("error");
        }
    }
}
