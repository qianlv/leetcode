/*
 * @lc app=leetcode id=970 lang=cpp
 *
 * [970] Powerful Integers
 */

#include <unordered_set>
#include <vector>

// @lc code=start
class Solution {
  public:
    std::vector<int> powerfulIntegers(int x, int y, int bound) {
        std::unordered_set<int> result;
        for (int i = 1; i < bound; i *= x) {
            for (int j = 1; i + j <= bound; j *= y) {
                result.insert(i + j);
                if (y == 1) {
                    break;
                }
            }
            if (x == 1) {
                break;
            }
        }
        return std::vector<int>{result.begin(), result.end()};
    }
};
// @lc code=end
