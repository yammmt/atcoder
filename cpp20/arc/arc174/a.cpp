#include <iostream>
#include <vector>

int main(void) {
    int n, c;
    std::cin >> n >> c;

    std::vector<long long> an;
    for(int i=0; i<n; i++) {
        long long a;
        std::cin >> a;
        an.push_back(a);
    }

    long long sum_max_before_mul = c>0? -999'999'999: 999'999'999;
    long long sum_max_prev = 0;
    for(int i=0; i<n; i++) {
        if (c > 0) {
            long long sum_cur = sum_max_prev > 0? sum_max_prev+an[i]: an[i];
            sum_max_before_mul = std::max(sum_max_before_mul, sum_cur);
            sum_max_prev = sum_cur;
        } else {
            long long sum_cur = sum_max_prev < 0? sum_max_prev+an[i]: an[i];
            sum_max_before_mul = std::min(sum_max_before_mul, sum_cur);
            sum_max_prev = sum_cur;
        }
    }
    // std::cout << "sum_max: " << sum_max_before_mul << std::endl;

    long long an_sum = 0;
    for(int i=0; i<n; i++) {
        an_sum += an[i];
    }
    // std::cout << "an_sum: " << an_sum << std::endl;

    long long ans = std::max(an_sum - sum_max_before_mul + c * sum_max_before_mul, an_sum);
    std::cout << ans << std::endl;

    return 0;
}
