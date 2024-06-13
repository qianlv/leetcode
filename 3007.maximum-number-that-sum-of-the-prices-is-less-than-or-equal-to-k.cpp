/*
 * @lc app=leetcode id=3007 lang=cpp
 *
 * [3007] Maximum Number That Sum of the Prices Is Less Than or Equal to K
 */

#include <iostream>

// @lc code=start
class Solution {
    const long long MAX_BIT = 50;

  public:
    long long findMaximumNumber(long long k, int x) {
        long long left = 1, right = (1LL << MAX_BIT);
        while (left < right) {
            long long mid = (left + right + 1) / 2;
            if (totalPrice(mid, x) <= k) {
                left = mid;
            } else {
                right = mid - 1;
            }
        }
        return left - 1;
    }

    long long totalPrice(long long num, int x) {
        long long price = 0;
        for (int i = 1; i * x < MAX_BIT; i++) {
            long long gap = 1LL << (i * x);
            // if (gap > num * 2) {
            //     break;
            // }
            long long n = num & (~(gap - 1));
            // std::cout << "gap = " << gap << std::endl;
            long long left = num & (gap - 1);
            price += (n >> 1);
            if (left > (gap >> 1)) {
                price += (left - (gap >> 1));
            }
        }
        // std::cout << "num = " << num << ", price = " << price << std::endl;
        return price;
    }
};
// @lc code=end

int main(int argc, const char **argv) {
    Solution sol;
    // std::cout << sol.totalPrice(7, 1) << std::endl;
    std::cout << sol.totalPrice(8, 1) << std::endl;
    std::cout << sol.totalPrice(11, 2) << std::endl;
    std::cout << sol.findMaximumNumber(7, 2) << std::endl;
    std::cout << sol.findMaximumNumber(3278539330613, 5) << std::endl;
}