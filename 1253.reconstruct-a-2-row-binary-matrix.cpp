/*
 * @lc app=leetcode id=1253 lang=cpp
 *
 * [1253] Reconstruct a 2-Row Binary Matrix
 */

#include <vector>

using std::vector;

// @lc code=start
class Solution {
  public:
    vector<vector<int>> reconstructMatrix(int upper, int lower, vector<int> &colsum) {
        vector<vector<int>> result(2);
        for (int sum : colsum) {
            if (sum == 2 && upper > 0 && lower > 0) {
                upper--;
                lower--;
                result[0].push_back(1);
                result[1].push_back(1);
            } else if (sum == 1) {
                if (upper >= lower && upper > 0) {
                    upper--;
                    result[0].push_back(1);
                    result[1].push_back(0);
                } else if (lower > upper && lower > 0) {
                    lower--;
                    result[0].push_back(0);
                    result[1].push_back(1);
                } else {
                    return {};
                }
            } else if (sum == 0) {
                result[0].push_back(0);
                result[1].push_back(0);
            } else {
                return {};
            }
        }
        return upper == 0 && lower == 0 ? result : vector<vector<int>>();
    }
};
// @lc code=end

int main(int argc, const char **argv) {}