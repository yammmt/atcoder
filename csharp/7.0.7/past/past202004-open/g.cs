using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main(string[] args)
    {
        int q = int.Parse(Console.ReadLine());
        var deque = new LinkedList<(char, long)>();
        for (int i = 0; i < q; i++)
        {
            string[] s = Console.ReadLine().Split();
            if (s[0] == "1")
            {
                // 挿入
                char c = char.Parse(s[1]);
                int x = int.Parse(s[2]);
                deque.AddLast((c, x));
            }
            else
            {
                // 削除と答えの出力
                long ans = 0;
                // 初期値 0 は保証されているはず...
                var removed = new long[26];

                var d = long.Parse(s[1]);
                long cur_removed = 0;
                while (cur_removed < d && deque.Count > 0)
                {
                    (char, long) cl = deque.First.Value;
                    deque.RemoveFirst();
                    var j = (int)cl.Item1 - 'a';
                    ans -= removed[j] * removed[j];
                    if (cl.Item2 + cur_removed >= d)
                    {
                        // 消せるだけ消して再代入
                        // 0 が入るがそのっま残しても大した影響はない
                        removed[j] += (d - cur_removed);
                        ans += removed[j] * removed[j];
                        deque.AddFirst((cl.Item1, cl.Item2 - (d - cur_removed)));
                        // 真面目に足しても同じだし直 break の方が賢いかも
                        cur_removed = d;
                    }
                    else
                    {
                        // 消しきれないので次に委ねる
                        removed[j] += cl.Item2;
                        ans += removed[j] * removed[j];
                        cur_removed += cl.Item2;
                    }
                }

                Console.WriteLine(ans);
            }
        }
    }
}
