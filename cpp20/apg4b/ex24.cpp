#include <iostream>

using namespace std;

// 以下に、24時間表記の時計構造体 Clock を定義する

struct Clock {
    int hour;
    int minute;
    int second;

    void set(int h, int m, int s) {
        hour = h;
        minute = m;
        second = s;
    }

    string to_str(void) {
        string ret;

        auto format = [](int n) {
            string r;
            if (n < 10) {
                r += "0";
            }
            r += to_string(n);
            return r;
        };

        ret += format(hour);
        ret += ":";
        ret += format(minute);
        ret += ":";
        ret += format(second);

        return ret;
    }

    void shift(int diff_second) {
        // 00:00:00 からの経過時間
        int sec_org = hour*60*60 + minute*60 + second;
        int sec_new = sec_org + diff_second;

        // 日付は考慮しない
        int sec_1day = 24*60*60;
        if (sec_new < 0) {
            sec_new += sec_1day;
            sec_new %= sec_1day;
        }

        hour = (sec_new / 3600) % 24;
        sec_new %= 3600;
        minute = sec_new / 60;
        sec_new %= 60;
        second = sec_new;
    }
};

// -------------------
// ここから先は変更しない
// -------------------

int main() {
    // 入力を受け取る
    int hour, minute, second;
    cin >> hour >> minute >> second;
    int diff_second;
    cin >> diff_second;

    // Clock構造体のオブジェクトを宣言
    Clock clock;

    // set関数を呼び出して時刻を設定する
    clock.set(hour, minute, second);

    // 時刻を出力
    cout << clock.to_str() << endl;

    // 時計を進める(戻す)
    clock.shift(diff_second);

    // 変更後の時刻を出力
    cout << clock.to_str() << endl;
}
