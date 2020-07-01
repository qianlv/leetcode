/*
 * Problem: https://leetcode.com/problems/cherry-pickup-ii/
 * DP
 * dp[i][j][k] 表示第i行，2个机器人分别为j, k列时值. 根据移动方式很容易写出转移方程
 */


#include <iostream>
#include <vector>
#include <algorithm>
#include <queue>

using namespace std;

class Solution {
public:
    int cherryPickup(const vector<vector<int>>& grid) {
        int row = grid.size();
        int col = grid[0].size();
        vector<vector<vector<int> > > dp(row, vector<vector<int>>(col, vector<int>(col, -1)));
        dp[0][0][col-1] = grid[0][col-1] + grid[0][0];

        vector<int> dire{-1, 0, 1};
        for (int i = 1; i < row; ++i) {
            for (int j = 0; j < col; ++j) {
                for (int k = 0; k < col; ++k) {
                    if (dp[i-1][j][k] == -1) {
                        continue;
                    }

                    for (int d1 : dire) {
                        int x = j + d1;
                        if (x < 0 || x >= col) {
                            continue;
                        }
                        for (int d2: dire) {
                            int y = k + d2;
                            if (y < 0 || y >= col) {
                                continue;
                            }
                            int cost = grid[i][x];
                            if (x != y) {
                                cost += grid[i][y];
                            }
                            dp[i][x][y] = max(dp[i][x][y], cost + dp[i-1][j][k]);
                        }
                    }
                }
            }
        }
        int ret = 0;
        for (int j = 0; j < col; ++j) {
            for (int k = 0; k < col; ++k) {
                ret = max(dp[row-1][j][k], ret);
            }
        }
        return ret;
    }
};

int main(int argc, char *argv[])
{
    Solution sol;
    std::cout << sol.cherryPickup({
            {3, 1, 1},
            {2, 5, 1},
            {1, 5, 5},
            {2, 1, 1},
            }) << std::endl;
    return 0;
}
