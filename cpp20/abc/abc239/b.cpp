#include <iostream>

#define LL long long

using namespace std;

int main(void) {
    LL x;
    cin >> x;

    if (x%10 < 0) {
        cout << x/10-1 << endl;
    } else {
        cout << x/10 << endl;
    }

    return 0;
}
