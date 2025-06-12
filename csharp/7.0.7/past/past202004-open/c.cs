using System;

class Program
{
    static void Main()
    {
        int n = int.Parse(Console.ReadLine());
        var grid = new char[n, 2 * n - 1];
        for (int i = 0; i < n; i++)
        {
            string line = Console.ReadLine();
            for (int j = 0; j < 2 * n - 1; j++)
            {
                grid[i, j] = line[j];
            }
        }
        (int, int)[] dir = { (1, -1), (1, 0), (1, 1) };

        for (int i = n - 2; i >= 0; i--)
        {
            var grid_new = new char[n, 2 * n - 1];
            Array.Copy(grid, grid_new, grid.Length);
            for (int j = 1; j < 2 * n - 2; j++)
            {
                if (grid[i, j] != '#')
                {
                    continue;
                }

                foreach ((int, int) d in dir)
                {
                    var ii = i + d.Item1;
                    var jj = j + d.Item2;
                    if (ii >= 0 && jj >= 0 && ii < n && jj < 2 * n - 1 && grid[ii, jj] == 'X')
                    {
                        grid_new[i, j] = 'X';
                    }
                }
            }
            grid = grid_new;
        }

        for (int i = 0; i < n; i++)
            {
                for (int j = 0; j < 2 * n - 1; j++)
                {
                    Console.Write(grid[i, j]);
                }
                Console.WriteLine();
            }
    }
}
