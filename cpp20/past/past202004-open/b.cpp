#include <iostream>

#define LL long long

using namespace std;

int main(void) {
  string s;
  cin >> s;

  int a = 0;
  int b = 0;
  int c = 0;
  for (const auto& ss: s) {
    switch (ss) {
      case ('a'):
        a++;
        break;
      case ('b'):
        b++;
        break;
      case ('c'):
        c++;
        break;
      default:
        break;
    }
  }

  if (a>b && a>c) {
    cout << "a" << endl;
  } else if (b>c && b>a) {
    cout << "b" << endl;
  } else {
    cout << "c" << endl;
  }

  return 0;
}
