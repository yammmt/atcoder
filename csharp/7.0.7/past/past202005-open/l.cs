using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main()
    {
        const int dummyIdx = 1_000_000;
        const int dummyT = -1;
        var n = int.Parse(Console.ReadLine());
        var kn = new List<int>();
        var tnk = new List<List<int>>();
        for (int i = 0; i < n; i++)
        {
            var line = Console.ReadLine().Split(' ').Select(int.Parse).ToList();
            // 要素外参照を防げるので番兵を入れておく
            // 実質三番目の要素まで見るので, 番兵は二つ入れる
            var len = line[0] + 2;
            kn.Add(len);
            var tk = new List<int>(len);
            for (int j = 0; j < line[0]; j++)
                tk.Add(line[j + 1]);
            tk.Add(dummyT);
            tk.Add(dummyT);
            tnk.Add(tk);
        }
        var m = int.Parse(Console.ReadLine());
        var am = Console.ReadLine().Split(' ').Select(int.Parse).ToArray();

        // 実装重い, もうちょっと減らしたいが
        // 手前から一番目/二番目の商品に対して, それぞれ降順ヒープを用意する
        // 追加はそれぞれの列に対して一番目/二番目の値を管理して進める
        // 雑に enqueue しても削除判定かかって消えるので動作はする, ループ回数が要素数の二倍にはなるが
        // 削除は while で未削除引くまで回してやればよいだけ

        // TODO: 長い...
        // 各列の 1/2 つ目要素
        var displayed = new (int first, int second)[n];
        for (int i = 0; i < n; i++)
        {
            displayed[i] = (0, kn[i] > 1 ? 1 : dummyIdx);
        }
        // 各列の挿入
        // ((k, t), 期限), 期限は unique, 降順
        // 比較関数作らずに符号反転して突っ込んだ方が高速のよう
        var pqFirst = new PriorityQueue<(int, int), int>(
            Comparer<int>.Create((x, y) => y.CompareTo(x))
        );
        var pqSecond = new PriorityQueue<(int, int), int>(
            Comparer<int>.Create((x, y) => y.CompareTo(x))
        );
        for (int i = 0; i < n; i++)
        {
            pqFirst.Enqueue((i, 0), tnk[i][0]);
            pqSecond.Enqueue((i, 0), tnk[i][0]);
            pqSecond.Enqueue((i, 1), tnk[i][1]);
        }

        var ans = new int[m];
        for (int i = 0; i < m; i++)
        {
            if (am[i] == 1)
            {
                while (pqFirst.TryDequeue(out (int k, int idx) item, out int _))
                {
                    if (item.idx != displayed[item.k].first)
                    {
                        continue;
                    }

                    ans[i] = tnk[item.k][item.idx];

                    // 二番目 -> 一番目
                    var iSecond = displayed[item.k].second;
                    displayed[item.k].first = iSecond;
                    pqFirst.Enqueue((item.k, iSecond), tnk[item.k][iSecond]);

                    // 三番目 -> 二番目
                    var iThird = iSecond + 1;
                    displayed[item.k].second = iThird;
                    pqSecond.Enqueue((item.k, iThird), tnk[item.k][iThird]);

                    break;
                }
            }
            else
            {
                while (pqSecond.TryDequeue(out (int k, int idx) item, out int _))
                {
                    if (item.idx != displayed[item.k].first && item.idx != displayed[item.k].second)
                    {
                        continue;
                    }

                    ans[i] = tnk[item.k][item.idx];

                    // 二番目 -> 一番目
                    // 棚の一番目は, 二番目を選択していない場合にのみ更新が必要
                    var iSecond = displayed[item.k].second;
                    if (item.idx == displayed[item.k].first)
                    {
                        displayed[item.k].first = iSecond;
                        pqFirst.Enqueue((item.k, iSecond), tnk[item.k][iSecond]);
                    }

                    // 三番目 -> 二番目
                    var iThird = iSecond + 1;
                    displayed[item.k].second = iThird;
                    pqSecond.Enqueue((item.k, iThird), tnk[item.k][iThird]);

                    break;
                }
            }
        }
        Console.WriteLine(string.Join("\n", ans));
    }
}
