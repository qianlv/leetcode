/*
 * Problems: https://leetcode.com/problems/find-critical-and-pseudo-critical-edges-in-minimum-spanning-tree/
 *
 * 使用 prim 求出MST的长度
 * critical: 删除一条边，使MST变大
 * pesudo critical: 让这条边的长度变为0，让这条边一定在MST中，如果这个MST的长度 + 这条边原始的长度 == 无修改的MST的长度，那么这条边就是 pesudo critical
 */

#include <iostream>
#include <vector>
#include <algorithm>
#include <queue>

using namespace std;


class Solution {
    using pair_t = std::pair<int, int>;
public:
    int prim(const std::vector<vector<pair_t> >& gh, int n, int exu, int exv, int start) {
        std::priority_queue<pair_t,

            std::vector<pair_t>,
                           std::greater<pair_t> > que;
        std::vector<bool> visited(n, false);
        std::vector<int> distance(n, -1);

        // cout << "start =" << start << endl;
        int sum = 0;
        distance[start] = 0;
        que.push({0, start});
        int cnt = 0;
        while (!que.empty()) {
            pair_t from = que.top();
            int u = from.second;
            int w = from.first;
            que.pop();
            if (visited[u]) {
                continue;
            }
            visited[u] = true;
            cnt += 1;
            sum += w;
            if (cnt == n) {
                break;
            }
            // std::cout << from.second << std::endl;
            for (const pair_t& to : gh[u]) {
                int v = to.first;
                int ew = to.second;
                if (visited[v] || (u == exu && v == exv) || (u == exv && v == exu)) {
                    continue;
                }
                if (distance[v] == -1 || distance[v] > ew) {
                    distance[v] = ew;
                    que.push({ew, v});
                }
            }
        }
        // cout << "cnt =" << cnt << endl;
        if (cnt != n) {
            return -1;
        }
        return sum;
    }
    vector<vector<int>> findCriticalAndPseudoCriticalEdges(int n, const vector<vector<int>>& edges) {
        std::vector<vector<pair_t> > gh(n, vector<pair_t>{});
        for (const auto& edge : edges) {
            gh[edge[0]].push_back({edge[1], edge[2]});
            gh[edge[1]].push_back({edge[0], edge[2]});
        }

        int sum = this->prim(gh, n, -1, -1, 0);
        //std::cout << sum << std::endl;
        vector<int> critical;
        vector<int> pseudo;
        for (size_t i = 0; i < edges.size(); ++i) {
            int dsum = this->prim(gh, n, edges[i][0], edges[i][1], 0);
            // cout << i << ":" << dsum << endl;
            if (dsum == -1 || dsum > sum) {
                critical.push_back(i);
            } else {
                pair_t* record = NULL;
                for (pair_t& to : gh[edges[i][0]]) {
                    if (to.first == edges[i][1]) {
                        record = &to;
                        to.second = 0;
                        break;
                    }
                }
                int msum = this->prim(gh, n, -1, -1, edges[i][0]);
                // cout << i << ":" << edges[i][0] << ":"<< msum << endl;
                if (msum + edges[i][2] == sum) {
                    pseudo.push_back(i);
                }
                record->second = edges[i][2];
            }
        }
        return {critical, pseudo};
    }

};

int main(int argc, char *argv[])
{
    Solution sol;
    // sol.findCriticalAndPseudoCriticalEdges(5, {
    //         {0,1,1},{1,2,1},{2,3,2},{0,3,2},{0,4,3},{3,4,3},{1,4,6}
    //         });
    sol.findCriticalAndPseudoCriticalEdges(4, {
            {0,1,1},{1,2,1},{2,3,1},{0,3,1},
            });
    return 0;
}
