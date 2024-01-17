/*
 * @lc app=leetcode id=2680 lang=cpp
 *
 * [2680] Maximum OR
 */
#include <algorithm>
#include <cstdint>
#include <cstdio>
#include <vector>

using std::greater;
using std::vector;

// @lc code=start
class Solution {
  public:
    static unsigned getHighestBit(unsigned num) {
        unsigned bit = 0;
        while (num > 0) {
            num >>= 1U;
            bit++;
        }
        return bit;
    }

    static uint64_t maximumOr(vector<int> &nums, unsigned k) {
        vector<uint64_t> post(nums.size() + 1, 0);
        for (int i = static_cast<int>(nums.size()) - 1; i >= 0; i--) {
            post[i] = post[i + 1] | static_cast<uint64_t>(nums[i]);
        }
        uint64_t ans = 0;
        uint64_t prefix = 0;
        for (int i = 0; i < nums.size(); i++) {
            auto cur = static_cast<uint64_t>(nums[i]);
            ans = std::max(ans, (cur << k) | prefix | post[i + 1]);
            prefix |= cur;
        }
        return ans;
    }
};
// @lc code=end

int main(int /*argc*/, const char ** /*argv*/) {
    Solution sol;
    vector<int> nums = {10, 8, 4};
    printf("%lu\n", sol.maximumOr(nums, 1));
}
