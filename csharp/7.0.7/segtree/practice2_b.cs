using System;
using System.Collections.Generic;
using System.Linq;

class Program {
    private class SegTree<T>
    {
        private int n;
        private int size;
        private T[] node;
        private T identityE;
        private Func<T, T, T> combineF;

        public SegTree(int n, T identityE, Func<T, T, T> combineF)
        {
            this.n = n;
            this.identityE = identityE;
            this.combineF = combineF;

            size = 1;
            while (size < n) size <<= 1;

            node = new T[size * 2];
            Array.Fill(node, identityE);
        }

        /// <summary>
        ///   [初期化] 配列の各要素を初期化する
        /// </summary>
        /// <param name="array">登録する要素の配列</param>
        public void build(T[] array)
        {
            if (array.Length < n)
                throw new ArgumentException("Array length is invalid");

            for (int i = 0; i < n; i++)
                {
                    node[i + size] = array[i];
                }

            for (int i = this.size - 1; i > 0; i--)
            {
                node[i] = combineF(node[i << 1], node[i << 1 | 1]);
            }
        }

        /// <summary>
        ///   [一点更新] 任意の位置の値を更新
        /// </summary>
        /// <param name="index">更新対象のインデックス</param>
        /// <param name="value">更新後の値</param>
        public void update(int index, T value)
        {
            int i = index + size;
            node[i] = value;
            while (i > 1)
            {
                i >>= 1;
                node[i] = combineF(node[i << 1], node[i << 1 | 1]);
            }
        }

        /// <summary>
        ///   [区間取得] 区間 [left, right) 内に `combineF` を順々に適用した結果
        /// </summary>
        /// <param name="left">閉区間の開始地点</param>
        /// <param name="right">開区間の終了地点</param>
        /// <returns>計算結果</returns>
        public T fold(int left, int right)
        {
            int l = left + size;
            int r = right + size;
            T valueL = identityE;
            T valueR = identityE;
            while (l < r)
            {
                if (l % 2 > 0)
                {
                    valueL = combineF(valueL, node[l]);
                    l++;
                }
                if (r % 2 > 0)
                {
                    r--;
                    valueR = combineF(node[r], valueR);
                }
                l >>= 1;
                r >>= 1;
            }
            return combineF(valueL, valueR);
        }
    }

    private static void Main()
    {
        var nq = Console.ReadLine().Split().Select(int.Parse).ToArray();
        var n = nq[0];
        var q = nq[1];
        var an = Console.ReadLine().Split().Select(long.Parse).ToArray();

        var st = new SegTree<long>(n, 0, (a, b) => a + b);
        st.build(an);

        for (int i = 0; i < q; i++)
        {
            var qq = Console.ReadLine().Split().Select(long.Parse).ToArray();
            if (qq[0] == 0)
            {
                var p = (int)qq[1];
                var x = qq[2];
                st.update(p, st.fold(p, p + 1) + x);
            }
            else
            {
                var l = (int)qq[1];
                var r = (int)qq[2];
                Console.WriteLine(st.fold(l, r));
            }
        }
    }
}