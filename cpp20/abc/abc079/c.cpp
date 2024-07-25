#include <iostream>

using namespace std;

int main(void) {
  int v;
  cin >> v;
  int abcd[4] = {0};
  for (auto i=0; i<4; i++) {
    abcd[4-i-1] = v%10;
    v /= 10;
  }

  auto is_op_add = [](int bit) {
    return (bit%2==1)? true: false;
  };

  auto bit_max = 2*2*2;
  for (auto bit=0; bit<bit_max; bit++) {
    auto b = bit;
    auto v = abcd[0];
    // 空文字列が保証される はず
    string ans;

    ans += to_string(abcd[0]);

    for (auto i=0; i<3; i++) {
      if (is_op_add(b)) {
        ans += "+";
        v += abcd[i+1];
      } else {
        ans += "-";
        v -= abcd[i+1];
      }
      b /= 2;

      ans += to_string(abcd[i+1]);
    }

    if (v == 7) {
      ans += "=7";
      cout << ans << endl;
      return 0;
    }
  }

  return 0;
}
