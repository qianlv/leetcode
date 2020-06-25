/*
 * Problem: https://leetcode.com/contest/biweekly-contest-28/problems/allocate-mailboxes/
 */

#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

class Solution {
public:
    int calCost(const vector<int>& houses, int i, int j) const {
        int cost = 0;
        while (i <= j) {
            int c = houses[i] - houses[j];
            if (c < 0) {
                c = -c;
            }
            cost += c;
            i++;
            j--;
        }
        return cost;
    }

    /*
     * dp[i][k] 表示 [0, i] 之间有 k mail 时的最小距离
     * 转移方程:
     *  dp[i][k] = dp[j][k-1] + [j+1, i] 之间有 1 mail 时的最小距离
     *             for j from 0 to i-1.
     */
    int minDistance(vector<int>& houses, int k) {
        sort(houses.begin(), houses.end());
        int n = houses.size();
        // 简单的优化，预先求出[i, j] 之间，有 1 mail 的最小距离
        vector<vector<int> > cache(n, vector<int>(n, 0));
        for (int i = 0; i < n; ++i) {
            for (int j = 0; j < n; ++j) {
                cache[i][j] = calCost(houses, i, j);
            }
        }

        vector<vector<int> > dp(n, vector<int>(k+1, -1));
        for (int i = 0; i < n; ++i) {
            dp[i][1] = cache[0][i];
        }

        for (int kk = 2; kk <= k; ++kk) {
            for (int i = 0; i < n; ++i) {
                for (int j = 0; j < i; ++j) {
                    // std::cout << j+1 << "-" << i << std::endl;
                    int cost = dp[j][kk-1] + cache[j+1][i];
                    if (dp[i][kk] == -1) {
                        dp[i][kk] = cost;
                    } else {
                        dp[i][kk] = std::min(dp[i][kk], cost);
                    }
                }
            }
        }
        return dp[n-1][k];
    }
};

int main(int argc, char *argv[])
{
    Solution sol;
    vector<int>houses{1, 4, 8, 10, 20};
    std::cout << sol.minDistance(houses, 3) << std::endl;
    vector<int>houses1{2,3,5,12,18};
    std::cout << sol.minDistance(houses1, 2) << std::endl;
    return 0;
}
