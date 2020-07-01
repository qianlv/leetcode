/*
 * Problems: https://leetcode.com/problems/probability-of-a-two-boxes-having-the-same-number-of-distinct-balls/
 * 根据 https://en.wikipedia.org/wiki/Multinomial_theorem
 */

#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

class Solution {
private:
    int sum;
public:
    double perm(const vector<int>& balls) {
        double ans = 1;
        int n = 1;
        for (size_t i = 0; i < balls.size(); ++i) {
            for (int j = 1; j <= balls[i]; ++j) {
                ans *= 1.0 * n / j;
                n += 1;
            }
        }
        return ans;
    }

    double dfs(vector<int>& balls, int depth, vector<int>& box1, vector<int>& box2, int s1, int s2) {
        if (s1 > sum / 2 || s2 > sum / 2) {
            return 0;
        } else if (depth == (int)balls.size()) {
            int c1(0), c2(0);
            for (int v : box1) {
                std::cout << v << " ";
                c1 += (v > 0);
            }
            std::cout << std::endl;
            for (int v : box2) {
                std::cout << v << " ";
                c2 += (v > 0);
            }
            std::cout << std::endl;
            if (c1 == c2) {
                return perm(box1) * perm(box2);
            }
            return 0;
        }

        double ans = 0;
        for (int i = 0; i <= balls[depth]; ++i) {
            box1[depth] = i;
            box2[depth] = balls[depth] - i;
            ans += dfs(balls, depth + 1, box1, box2, s1 + i, s2 + balls[depth] - i);
        }
        return ans;
    }

    double getProbability(vector<int>& balls) {
        sum = 0;
        for (int v : balls) {
            sum += v;
        }
        vector<int> box1(balls.size(), 0), box2(balls.size(), 0);
        return dfs(balls, 0, box1, box2, 0, 0) / perm(balls);
    }
};

int main(int argc, char *argv[])
{
    Solution sol;
    vector<int> balls{1, 2, 1, 2};
    // cout << sol.getProbability({3, 2, 1}) << endl;
    // cout << sol.getProbability({1, 2, 1, 2}) << endl;
    // cout << sol.getProbability({2, 1, 1}) << endl;
    cout << sol.getProbability(balls) << endl;
    // cout << sol.getProbability({6, 6, 6, 6, 6, 6, 6}) << endl;
    // cout << sol.getProbability({6, 6, 6, 6, 6, 6, 6, 6}) << endl;
    return 0;
}
