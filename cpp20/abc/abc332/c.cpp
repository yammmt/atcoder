#include <iostream>

using namespace std;

int main(void) {
  int n, m;
  cin >> n >> m;
  string s;
  cin >> s;

  int ans = 0;
  int plain_stock = m;
  int plain_used = 0;
  int atc_stock = 0;
  int atc_used = 0;

  auto use_shirt = [](int& stock, int& used) {
    stock--;
    used++;
  };

  auto buy = [&ans, &atc_used](){
    ans++;
    atc_used++;
  };

  for (const auto& c: s) {
    if (c == '1') {
      // 無地があれば無地
      if (plain_stock > 0) {
        use_shirt(plain_stock, plain_used);
      } else if (atc_stock > 0) {
        use_shirt(atc_stock, atc_used);
      } else {
        buy();
      }
    } else if (c == '2') {
      // AtC
      if (atc_stock == 0) {
        buy();
      } else {
        use_shirt(atc_stock, atc_used);
      }
    } else if (c == '0') {
      // 洗濯
      plain_stock += plain_used;
      plain_used = 0;
      atc_stock += atc_used;
      atc_used = 0;
    }
  }

  cout << ans << endl;

  return 0;
}
