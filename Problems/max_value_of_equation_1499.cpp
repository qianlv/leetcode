/*
 * Problem: https://leetcode.com/problems/max-value-of-equation/
 * Date: 2020-06-29 11:09
 */
#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

/*
 * Yi + Yj + |Xi - Xj| = (Yi - Xi) + (Yj + Xj)
 * 分解为2部分，然后为一个窗口保证 |Xi - Xj| <= k,
 * 然后通过RMQ求出[i+1, j] 之间 (Yk + Xk) 的最大值,
 * 这样遍历完数组，就是结果了.
 */
class Solution {
private:
    vector<vector<int> > rmq;
    int level;

public:
    void initRmq(const vector<int>& diffPoints) {
        int n = diffPoints.size();
        rmq.push_back(diffPoints);
        vector<int> tmp(diffPoints.size(), 0);
        for (int i = 1; (1 << i) <= n; ++i) {
            rmq.push_back(vector<int>(n, 0));
            int off = (1 << (i-1));
            for (int j = 0; j < n - off; ++j) {
                rmq[i][j] = max(rmq[i-1][j], rmq[i-1][j+off]);
            }
        }
        level = rmq.size();
    }

    int maxRmp(int l, int r) {
        int len = r - l + 1;
        int off = 0;
        while (len > (1 << (off + 1))) {
            off ++;
        }
        return max(rmq[off][l], rmq[off][r + 1 - (1 << off)]);
    }


    int findMaxValueOfEquation(vector<vector<int>>& points, int k) {
        vector<int> sumPoints;
        vector<int> diffPoints;
        for (const auto& point : points) {
            sumPoints.push_back(point[0] + point[1]);
            diffPoints.push_back(point[1] - point[0]);
        }
        initRmq(sumPoints);
        int n = points.size();
        int i, j = 1;
        int maxValue = 0;
        bool noSetMax = false;
        for (i = 0; i < n; ++i) {
            while (j < n && points[j][0] - points[i][0] <= k) {
                j += 1;
            }
            if (i < j-1) {
                std::cout << i+1 << "-" << j-1 << std::endl;
                std::cout << diffPoints[i] << std::endl;
                std::cout << maxRmp(i+1, j-1) << std::endl;
                if (!noSetMax) {
                    maxValue = diffPoints[i] + maxRmp(i+1, j-1);
                    noSetMax = true;
                } else {
                    maxValue = max(maxValue, diffPoints[i] + maxRmp(i+1, j-1));
                }
            }
        }
        return maxValue;
    }

};

int main(int argc, char *argv[])
{
    Solution sol;
    vector<vector<int>> points {
        {-19, 9}, {-15, -19}, {-5, -8},
    };
    std::cout << sol.findMaxValueOfEquation(points, 10) << std::endl;
    return 0;
}
