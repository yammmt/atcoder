#include <iostream>

#define LL long long

using namespace std;

int main(void) {
  // 問題文が難解

  string s;
  cin >> s;

  auto passed = [s](string a) {
    if (a.length() > s.length()) {
      return 0;
    }

    for (int i=0; i<s.length()-a.length()+1; i++) {
      if (a.length()==1 && (a[0]=='.' || a[0]==s[i])) {
        // cout << "  i: " << i << endl;
        return 1;
      }

      if (a[0]!='.' && a[0]!=s[i]) {
        continue;
      }

      int j = i+1;
      if (a.length()==2 && (a[1]=='.' || a[1]==s[j])) {
        // cout << "  j: " << j << endl;
        return 1;
      }
      if (a[1]!='.' && a[1]!=s[j]) {
        continue;
      }

      int k = j + 1;
      if (a[2]=='.' || a[2]==s[k]) {
        // cout << "k: " << k << endl;
        return 1;
      }
    }

    return 0;
  };

  string strs = "abcdefghijklmnopqrstuvwxyz.";
  int ans = 0;

  for (int i=0; i<strs.length(); i++) {
    for (int j=0; j<strs.length(); j++) {
      for (int k=0; k<strs.length(); k++) {
        string a = "";
        a += strs[i];
        a += strs[j];
        a += strs[k];
        ans += passed(a);
      }
    }
  }

  for (int i=0; i<strs.length(); i++) {
    for (int j=0; j<strs.length(); j++) {
      string a = "";
      a += strs[i];
      a += strs[j];
      // cout << a << endl;
      ans += passed(a);
    }
  }

  for (int i=0; i<strs.length(); i++) {
    string a = "";
    a += strs[i];
    ans += passed(a);
  }

  cout << ans << endl;

  return 0;
}
