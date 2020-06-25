/*
 * Problem: https://leetcode.com/contest/weekly-contest-193/problems/kth-ancestor-of-a-tree-node/
 */
#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

/*
 * 由于大量的查询，There will be at most 5*10^4 queries.
 * 所以查询祖先，需要预处理，powerAncestor[i][n], i 表示 2^i 距离的祖先是谁。
 * 这样对于k距离的查询，就可以按二进制预处理的2^i距离进行分解查询，
 * 例如9(1001), 那么查询2^4 + 2^0, node = powerAncestor[4][node], node = powerAncestor[0][node] 就为结果
 *
 * 查询效率即为 logn.
 */
class TreeAncestor {
private:
    std::vector<std::vector<int> > powerAncestor;
    int size;
public:
    TreeAncestor(int n, vector<int>& parent) {
        for (int i = 0; (1 << i) <= n; ++i) {
            powerAncestor.push_back({});
        }
        powerAncestor[0] = parent;

        for (int i = 1; (1 << i) <= n; ++i) {
            vector<int> ancestors;
            for (int j = 0; j < n; ++j) {
                int ancestor = powerAncestor[i-1][j];
                if (ancestor != -1) {
                    ancestor = powerAncestor[i-1][ancestor];
                }
                ancestors.push_back(ancestor);
            }
            powerAncestor[i] = ancestors;
            // std::cout << i << std::endl;
            // for (int j = 0; j < n; ++j) {
            //     std::cout << powerAncestor[i][j] << " ";
            // }
            // std::cout << std::endl;
        }
        size = powerAncestor.size();
    }
    
    int getKthAncestor(int node, int k) {
        for (int i = size - 1; i >= 0; --i) {
            if (k & (1 << i)) {
                node = powerAncestor[i][node];
                if (node == -1) {
                    break;
                }
            }
        }
        return node;
    }
};

/**
 * Your TreeAncestor object will be instantiated and called as such:
 * TreeAncestor* obj = new TreeAncestor(n, parent);
 * int param_1 = obj->getKthAncestor(node,k);
 */
int main(int argc, char *argv[])
{
    vector<int> parent1 = {-1, 0, 0, 1, 1, 2, 2};
    TreeAncestor* treeAncestor = new TreeAncestor(7, parent1);
    std::cout << treeAncestor->getKthAncestor(3, 1) << std::endl;
    std::cout << treeAncestor->getKthAncestor(5, 2) << std::endl;
    std::cout << treeAncestor->getKthAncestor(6, 3) << std::endl;

    return 0;
}
