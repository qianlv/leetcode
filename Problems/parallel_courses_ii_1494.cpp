/*
 * Problems: https://leetcode.com/problems/parallel-courses-ii/
 */
#include <iostream>
#include <vector>
#include <queue>
#include <list>

using namespace std;

/*
 * DFS 搜索, 1 <= n <= 15
 * 用 1 << 15 大小数组记录所有的状态，然后加速搜索.
 */
class Solution {
public:
    int minNumberOfSemesters(int n, vector<vector<int>>& dependencies, int k) {
        vector<vector<int> > gh(n, vector<int>{});
        vector<int> inDegrees(n, 0);
        for (auto& pair : dependencies) {
            inDegrees[pair[1]-1] += 1;
            gh[pair[0]-1].push_back(pair[1]-1);
        }
        vector<int> visited(1 << n, n+1);
        dfs(gh, inDegrees, visited, n, k, 0, 0);
        return visited[(1 << n) - 1];
    }


    void dfs(vector<vector<int>> &gh, vector<int>& inDegrees,  vector<int>& visited, int n, int k, int dis, int state) {
        if (visited[state] > dis) {
            visited[state] = dis;
        } else {
            return;
        }

        // std::cout << "--------------- " << state << std::endl;
        vector<int> vextexs;
        for (int i = 0; i < n; ++i) {
            // std::cout << i << std::endl;
            if (!(state & (1 << i))) {
                // std::cout << "stat: " << i << std::endl;
                if (inDegrees[i] == 0) {
                    // std::cout << "--" << i << std::endl;
                    vextexs.push_back(i);
                }
            }
        }

        if (vextexs.size() == 0) {
            // std::cout << "+++++++++++++ " << state << std::endl;
            return;
        }

        if ((int)vextexs.size() <= k) {
            run(gh, inDegrees, visited, 
                vextexs, (1 << vextexs.size()) - 1, n, k, dis, state);
        } else {
            for (int s = 0; s < (1 << vextexs.size()); s++) {
                int cnt = 0;
                int ts = s;
                while (ts) {
                    cnt += (ts & 1);
                    ts >>= 1;
                }
                if (cnt != k) {
                    continue;
                }
                // std::cout << "chosse = " << s << std::endl;
                run(gh, inDegrees, visited, vextexs, s, n, k, dis, state);
            }
        }
        // std::cout << "+++++++++++++ " << state << std::endl;
    }

    void run(vector<vector<int>> &gh, vector<int>& inDegrees,  vector<int>& visited,
             vector<int>& vextexs, int s, int n, int k, int dis, int state) {
        // std::cout << "s = " << s << std::endl;
        for (int i = 0; i < (int)vextexs.size(); ++i) {
            if (s & (1 << i)) {
                int v = vextexs[i];
                // std::cout << "v = " << v << std::endl;
                for (int j = 0; j < (int)gh[v].size(); ++j) {
                    inDegrees[gh[v][j]] -= 1;
                    // std::cout << "in = " << gh[v][j] << ":" << inDegrees[gh[v][j]] << std::endl;
                }
                state |= (1 << v);
            }
        }
        dfs(gh, inDegrees, visited, n, k, dis+1, state);
        for (int i = 0; i < (int)vextexs.size(); ++i) {
            if (s & (1 << i)) {
                int v = vextexs[i];
                for (int j = 0; j < (int)gh[v].size(); ++j) {
                    inDegrees[gh[v][j]] += 1;
                }
                state &= (~(1 << v));
            }
        }
    }
};

int main(int argc, char *argv[])
{
    Solution sol;
    vector<vector<int> > dependencies {
    };
    std::cout << sol.minNumberOfSemesters(11, dependencies, 2) << std::endl;
    return 0;
}
