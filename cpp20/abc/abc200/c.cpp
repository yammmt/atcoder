#include <algorithm>
#include <iostream>
#include <vector>

int main(void) {
    int n;
    std::cin >> n;

    std::vector<int> an;
    for(int i=0; i<n; i++) {
        int a;
        std::cin >> a;
        an.push_back(a%200);
    }

    // 自身よりうしろの数の出現数を管理する
    int ringos[200] = {0};
    std::for_each(an.begin(), an.end(), [&ringos](int a){
        ringos[a]++;
    });

    long long ans = 0;
    for(int i=0; i<n; i++) {
        ringos[an[i]]--;
        ans += ringos[an[i]];
    }

    std::cout << ans << std::endl;

    return 0;
}
