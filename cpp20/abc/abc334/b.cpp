// ARC と間違えた？少なくとも D よりは難しい出題
// 頻出でない言語仕様まで要求している
//   => ABC239 - B もだがこっちは解説が細かい
//      https://atcoder.jp/contests/abc239/editorial/3390?lang=ja

#include <iostream>

#define LL long long

using namespace std;

LL div_floor(LL a, LL b) {
    LL d = a / b;
    if (a%b < 0) {
        return d - 1;
    } else {
        return d;
    }
}

LL div_ceil(LL a, LL b) {
    LL d = a/b;
    if (a%b > 0) {
        return d+1;
    } else {
        return d;
    }
}

int main(void) {
    LL a, m, l, r;
    cin >> a >> m >> l >> r;

    // 読みやすくするため, すべての値を非負としたかった
    // が, 原点 A と区間との間隔がずれるので WA になる
    // 例えば `0 10 -9 -9` で 1 が返ってしまう

    // 区間 [l, r] 内の要素数は r-l+1
    // 区間を正しくするには l は切り上げ, r は切り捨てをしなくてはならない
    // C++ では整数の割り算の丸めの方向は 0 に近い側であるため,
    // 例えば r<0 に対し r/m が切り上げられてしまうのが厄介
    // さらには切り上げも, 例えば -6/3 は (a+b-1)/b すると -1 になってしまう

    // 原点を揃えることで, M の整数倍位置にツリーが置かれることが保証できる
    l -= a;
    r -= a;

    // M に置いて左右両側にできるだけ置く
    auto rt = div_floor(r, m);
    auto lt = div_ceil(l, m);

    auto ans = rt-lt+1;
    cout << ans << endl;

    return 0;
}
