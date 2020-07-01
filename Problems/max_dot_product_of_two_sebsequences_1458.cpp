/*
 * Problem: https://leetcode.com/problems/max-dot-product-of-two-subsequences/
 * DP
 * dp[i][j] [0, i] 与 [0, j] 2个序列的最大点积.
 */


#include <iostream>
#include <vector>
#include <algorithm>
#include <queue>

using namespace std;

class Solution {
public:
    int maxDotProduct(vector<int>& nums1, vector<int>& nums2) {
        int n = nums1.size();
        int m = nums2.size();
        vector<vector<int> > dp(n, vector<int>(m, 0));
        for (int i = 0; i < n; ++i) {
            for (int j = 0; j < m; ++j) {
                dp[i][j] = nums1[i] * nums2[j];
                if (i-1 > 0) {
                    dp[i][j] = max(dp[i-1][j], dp[i][j]);
                }
                if (j - 1 > 0) {
                    dp[i][j] = max(dp[i][j-1], dp[i][j]);
                }
                if (i - 1 > 0 && j - 1 > 0) {
                    dp[i][j] = max(dp[i][j], nums1[i-1] * nums2[j-1] + dp[i-1][j-1]);
                    dp[i][j] = max(dp[i][j], dp[i-1][j-1]);
                }
            }
        }
        return dp[n-1][m-1];
    }
};
