using System;
using System.Linq;

class Program
{
    static void Main()
    {
        var n = long.Parse(Console.ReadLine());
        var an = Console.ReadLine().Split().Select(long.Parse).ToArray();
        var an2 = an.Concat(an).ToArray();
        var scoreSum = an.Sum();

        // しゃくとり
        // 要素の重複は考えなくてよい？勝手に弾かれそう
        var ans = long.MaxValue;
        var scoreCur = 0L;
        var right = 0;
        for (int left = 0; left < 2 * n; left++)
        {
            while (right < 2 * n && scoreCur <= scoreSum / 2)
            {
                scoreCur += an2[right];
                // 超える直前の更新はここで入る
                ans = Math.Min(ans, Math.Abs(2 * scoreCur - scoreSum));
                right++;
            }
            // Console.WriteLine("left: " + left + ", right: " + right);

            // 超えた直後の更新はここで入る
            ans = Math.Min(ans, Math.Abs(2 * scoreCur - scoreSum));
            scoreCur -= an2[left];
        }

        Console.WriteLine(ans);
    }
}
