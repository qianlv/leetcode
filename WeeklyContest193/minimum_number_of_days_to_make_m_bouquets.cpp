/*
 * Problem: https://leetcode.com/contest/weekly-contest-193/problems/minimum-number-of-days-to-make-m-bouquets/
 *
 * 思路就是O(n) 判断 指定day 情况下是否可以满足条件
 * day 通过二分来确定，从而在O(nlogn)情况下解决
 */

#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

class Solution {
public:
    bool meetCondition(const vector<int>& bloomDay, int m, int k, int day) {
        int n = bloomDay.size();
        int count = 0;
        int nBouquet = 0;
        for (int i = 0; i < n; ++i) {
            if (bloomDay[i] - day <= 0) {
                count += 1;
                if (count >= k) {
                    nBouquet += 1;
                    count -= k;
                }
            } else {
                count = 0;
            }
        }
        return nBouquet >= m;
    }

    int minDays(vector<int>& bloomDay, int m, int k) {
        if (bloomDay.empty()) {
            return -1;
        }

        int minDay, maxDay;
        minDay = bloomDay[0];
        maxDay = bloomDay[0];
        int n = bloomDay.size();
        for (int i = 1; i < n; ++i) {
            minDay = min(minDay, bloomDay[i]);
            maxDay = max(maxDay, bloomDay[i]);
        }

        int left = minDay;
        int right = maxDay + 1;
        while (left < right) {
            int day = left + (right - left) / 2;
            if (!meetCondition(bloomDay, m, k, day)) {
                left = day + 1;
            } else {
                right = day;
            }
        }
        if (right == maxDay + 1) {
            return -1;
        }
        return right;
    }
};

int main(int argc, char *argv[])
{
    vector<int> bloomDay{1,10,3,10,2};
    Solution sol;
    std::cout << sol.minDays(bloomDay, 3, 2) << std::endl;
    return 0;
}
