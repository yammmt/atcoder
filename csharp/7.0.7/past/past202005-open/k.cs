using System;

class Program
{
    static void Main()
    {
        const int none = -1;

        var nq = Console.ReadLine().Split(' ');
        var n = int.Parse(nq[0]);
        var q = int.Parse(nq[1]);

        // 愚直に考えると, 各机の上にコンテナを線形リストでもつ
        // 移動開始時に机の情報が与えられるので, コンテナがどこにあるかは机を辿れば O(N) でわかる
        // 愚直だと O(NQ) になる. でもコンテナ初期値の都合で高さが均されるのでは？
        // 高さ見る回数が高々 2Q 回かそこらに収まりそう, 本当に？

        var containerLower = new int[n];
        Array.Fill(containerLower, none);
        var containerHighest = new int[n];
        for (int i = 0; i < n; i++)
            containerHighest[i] = i;

        for (int i = 0; i < q; i++)
        {
            var ftx = Console.ReadLine().Split(' ');
            var f = int.Parse(ftx[0]) - 1;
            var t = int.Parse(ftx[1]) - 1;
            var x = int.Parse(ftx[2]) - 1;

            // 移動先の最上位
            var highestPrev = containerHighest[t];
            containerHighest[t] = containerHighest[f];

            // 移動元の最上位
            containerHighest[f] = containerLower[x];

            // 移動先のひとつ下
            containerLower[x] = highestPrev;
        }

        // 一度全体を舐めてコンテナの位置を取得する
        var ans = new int[n];
        for (int i = 0; i < n; i++)
        {
            var j = containerHighest[i];
            while (j != none)
            {
                ans[j] = i + 1;
                j = containerLower[j];
            }
        }
        Console.WriteLine(string.Join("\n", ans));
    }
}
