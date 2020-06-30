/*
 * Problem: https://leetcode.com/problems/paint-house-iii/
 */

#include <iostream>
#include <vector>
#include <algorithm>
#include <climits>

using namespace std;

class Solution {
public:
    void dfs(const vector<int>& houses, const vector<vector<int>> &cost, int m, int n, int target,
            vector<vector<vector<int> > >& dp, int color, int depth) {

    }

    int minCost(const vector<int>& houses, const vector<vector<int>>& cost, int m, int n, int target) {
        int MAX = 10000000;
        vector<vector<vector<int> > > dp(m+1, vector<vector<int>> (n+1, vector<int>(target+1, MAX))); // m x n x m

        if (houses[0] == 0) {
            for (int c = 1; c <= n; ++c) {
                dp[1][c][1] = cost[0][c-1];
            }
        } else {
            dp[1][houses[0]][1] = cost[0][houses[0]-1];
        }

        // for (int i = 2; i <= m; ++i) {

        //     for (int c = 1; c <= n; ++c) {
        //         for (int k = 1; k <= target; ++k) {
        //             if (houses[i-1] == 0 || c == houses[i-1]) {
        //                 for (int prec = 1; prec <= n; ++prec) {
        //                     if (prec == c) {
        //                         dp[i][c][k] = min(dp[i][c][k], dp[i-1][prec][k] + cost[i-1][c-1]);
        //                     } else if (k >= 2) {
        //                         dp[i][c][k] = min(dp[i][c][k], dp[i-1][prec][k-1] + cost[i-1][c-1]);
        //                     }
        //                 }
        //             }
        //         }
        //     }
        // }


        for (int i = 2; i <= m; ++i) {
            for (int k = 1; k <= target; ++k) {
                // 预处理, 后面我们对于颜色c，求 dp[i-1][k-1][1...n] 的最小值，这里1...n 需要不包含c,
                // 这样对于当前颜色c可以做单独一段，
                vector<int> prefixmin(n+2, MAX), suffixmin(n+2, MAX);
                for (int c = 1; c <= n; ++c) {
                    prefixmin[c] = min(prefixmin[c-1], dp[i-1][c][k-1]);
                }
                for (int c = n; c >= 1; --c) {
                    suffixmin[c] = min(suffixmin[c-1], dp[i-1][c][k-1]);
                }

                for (int c = 1; c <= n; ++c) {
                    if (houses[i-1] == 0 || houses[i-1] == c) {
                        dp[i][c][k] = min(dp[i][c][k], dp[i-1][c][k] + cost[i-1][c-1]);
                       
                        if (k >= 2) {
                            /// std::cout << i << "|" << k << "|" << c << std::endl;
                            dp[i][c][k] = min(dp[i][c][k], min(prefixmin[c-1], suffixmin[c+1]) + cost[i-1][c-1]);
                        }
                    }
                }
            }
        }
        // this->dfs(houses, cost, m, n, target, dp, 0, 1);

        int minc = MAX;
        for (int c = 1; c <= n; ++c) {
            minc = min(minc, dp[m][c][target]);
        }
        if (minc >= MAX) {
            return -1;
        }
        for (int i = 0; i < m; ++i) {
            if (houses[i] != 0) {
                minc -= cost[i][houses[i]-1];
            }
        }
        return minc;
    }

};

int main(int argc, char *argv[])
{
    Solution sol;
    int ret =sol.minCost({0, 0, 0, 0, 0},
                {
                    {1, 10},
                    {10, 1},
                    {10, 1},
                    {1, 10},
                    {5, 1},
                },
                5, 2, 3
            );
    std::cout << ret << std::endl;
    return 0;
}
