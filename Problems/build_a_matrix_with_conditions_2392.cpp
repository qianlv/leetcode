// https://leetcode.com/problems/build-a-matrix-with-conditions/
//
// Topological Sort

#include <iostream>
#include <vector>
#include <list>
#include <stack>

using namespace std;

using Mat = vector<vector<int>>;
using ListIter = list<int>::iterator;

class Solution {
public:
    Mat buildMatrix(
        int k, const Mat& rowConditions, const Mat& colConditions) {
        pair<vector<int>, vector<int>> row = buildCondition(rowConditions, k);
        pair<vector<int>, vector<int>> col = buildCondition(colConditions, k);
        if (row.first.empty() || col.first.empty()) {
            return {};
        }
        Mat mat(k, vector<int>(k, 0));
        for (size_t i = 0; i < row.first.size(); ++i) {
            int v = row.first[i];
            int j = col.second[v];
            mat[i][j] = v;
        }
        return mat;
    }

    pair<vector<int>, vector<int>> buildCondition(const Mat& conditions, int k) {
        vector<list<int>> gh(k+1, std::list<int>{});
        vector<int> indegrees(k+1, 0);
        for (auto& cond : conditions) {
            gh[cond[0]].push_back(cond[1]);
            indegrees[cond[1]] += 1;
        }

        vector<int> indexs;
        vector<int> v2indexs(k+1, -1);
        stack<int> st;
        for (int i = 1; i < k+1; ++i) {
            if (indegrees[i] == 0) {
                st.push(i);
            }
        }

        while (!st.empty()) {
            int from = st.top();
            // std::cout << from << " ";
            indexs.push_back(from);
            v2indexs[from] = indexs.size() - 1;

            st.pop();
            for (int to : gh[from]) {
                indegrees[to] -= 1;
                if (indegrees[to] == 0) {
                    st.push(to);
                }
            }
        }

        // std::cout << std::endl;

        if (indexs.size() != k) {
            return {{}, {}};
        }

        return {indexs, v2indexs};
    }
};

int main(int argc, char *argv[])
{
    Solution sol;
    // Mat mat = sol.buildMatrix(3, {{1, 2}, {3, 2}}, {{2, 1}, {3, 2}});
    Mat mat = sol.buildMatrix(
        8,
        {{1,2},{7,3},{4,3},{5,8},{7,8},{8,2},{5,8},{3,2},{1,3},{7,6},{4,3},{7,4},{4,8},{7,3},{7,5}},
        {{5,7},{2,7},{4,3},{6,7},{4,3},{2,3},{6,2}}
    );
    for (auto row : mat) {
        for (int v : row) {
            std::cout << v << " ";
        }
        std::cout << std::endl;
    }
    return 0;
}
