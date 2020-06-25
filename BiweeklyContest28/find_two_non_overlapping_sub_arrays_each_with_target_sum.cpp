/*
 * Problem: https://leetcode.com/contest/biweekly-contest-28/problems/find-two-non-overlapping-sub-arrays-each-with-target-sum/
 */
#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

class Solution {
public:
    /*
     * minLens[i] 数组用于记录 [0, i)范围内 sum == target 的所有 sub-arrays 的最小长度.
     * 这样每次发现一个 [i, j] sub-arrays, 就可以通过  j - i + 1 + minLens[i] 来更新
     * the minimum sum of the lengths of the two required sub-arrays
     */
    int minSumOfLengths(vector<int>& arr, int target) {
        vector<int> minLens(arr.size()+1, arr.size() + 1);
        int n = arr.size();
        int i = 0, j = 0;
        int sum = 0;
        int minLen = arr.size() + 1;
        while (i < n && j < n) {
            sum += arr[j];
            if (j > 0) {
                minLens[j] = min(minLens[j-1], minLens[j]);
            }
            // std::cout << i << "-" << j << "=" << sum << std::endl;
            if (sum == target) {
                minLens[j+1] = min(minLens[j+1], min(minLens[j], j - i + 1));
                // std::cout << minLens[j+1] << std::endl;
                minLen = min(minLen, j - i + 1 + minLens[i]);
            } 

            if (sum <= target) {
                j ++;
            } else {
                sum -= arr[i];
                sum -= arr[j];
                i++;
            }
        }
        if (minLen == n + 1) {
            return -1;
        }
        return minLen;
    }
};

int main(int argc, char *argv[])
{
    Solution sol;
    vector<int> arr1{4,3,2,6,2,3,4};
    std::cout << sol.minSumOfLengths(arr1, 6) << std::endl;
    return 0;
}
