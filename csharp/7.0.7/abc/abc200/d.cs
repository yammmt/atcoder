using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main()
    {
        void PrintSet(List<int> indices)
        {
            Console.Write(indices.Count);
            foreach (var index in indices)
                Console.Write(" " + index);
            Console.WriteLine();
        }

        var n = int.Parse(Console.ReadLine());
        var an = Console.ReadLine().Split().Select(s => int.Parse(s) % 200).ToArray();

        // 鳩の巣原理 (pigeonhole principle)
        // 2^8 = 256, 全部含めないパターンを抜いても 255 個
        // n <= 8 なら愚直に全部見て, そうでなければ先頭 8 つと末尾 8 つとで鳩の巣原理でよさそう
        // 先頭/末を分ける必要もないか

        var pigeons = an.Take(Math.Min(8, n)).ToArray();
        var groupSize = pigeons.Length;

        var ht = new Dictionary<int, List<int>>();
        for (int i = 1; i < (1 << groupSize); i++)
        {
            var hole = 0;
            var iUsed = new List<int>();
            for (int j = 0; j < groupSize; j++)
            {
                if ((i & (1 << j)) > 0)
                {
                    hole += an[j];
                    hole %= 200;
                    iUsed.Add(j + 1);
                }
            }

            if (ht.ContainsKey(hole))
            {
                Console.WriteLine("Yes");
                PrintSet(ht[hole]);
                PrintSet(iUsed);
                return;
            }
            else
            {
                ht.Add(hole, iUsed);
            }
        }

        Console.WriteLine("No");
    }
}