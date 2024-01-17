#include <algorithm>
#include <cstddef>
#include <cstdio>
#include <vector>

/*
 * @lc app=leetcode id=870 lang=cpp
 *
 * [870] Advantage Shuffle
 */

// @lc code=start
class Solution {
  public:
    std::vector<int> advantageCount(std::vector<int> &nums1, std::vector<int> &nums2) {
        std::vector<int> nums1_index(nums1.size());
        std::vector<int> nums2_index(nums2.size());
        size_t len = nums1.size();
        for (int i = 0; i < len; ++i) {
            nums1_index[i] = i;
            nums2_index[i] = i;
        }

        std::sort(nums1_index.begin(), nums1_index.end(),
                  [&](int a, int b) { return nums1[a] < nums1[b]; });
        std::sort(nums2_index.begin(), nums2_index.end(),
                  [&](int a, int b) { return nums2[a] < nums2[b]; });
        std::vector<int> result(nums1.size(), -1);
        size_t last_index = len - 1;
        for (size_t i = 0, j = 0; i < len && j < len; ++i, ++j) {
            size_t index2 = nums2_index[i];
            size_t index1 = nums1_index[j];
            while (nums2[index2] >= nums1[index1] && j < len) {
                // printf("%d\n", nums1[index1]);
                result[nums2_index[last_index]] = nums1[index1];
                last_index--;
                ++j;
                if (j >= len) {
                    break;
                }
                index1 = nums1_index[j];
            }
            result[index2] = nums1[index1];
        }
        return result;
    }
};
// @lc code=end

int main(int argc, const char **argv) {
    Solution sol;
    std::vector<int> nums1 = {0};
    std::vector<int> nums2 = {0};
    auto result = sol.advantageCount(nums1, nums2);
    for (auto &&val : result) {
        printf("%d ", val);
    }
    printf("\n");
}