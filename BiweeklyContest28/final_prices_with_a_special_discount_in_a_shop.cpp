/*
 * Problem: https://leetcode.com/contest/biweekly-contest-28/problems/final-prices-with-a-special-discount-in-a-shop/
 */

#include <iostream>
#include <vector>
#include <stack>

using namespace std;

class Solution {
public:
    /*
     * 这里 stack<int> st 中index对应的 price[index] 值是递增，原因:
     * 1. 如果st 有相对递减的情况，比如 i < j and prices[i] >= prices[j] 等情况
     * 2. 和 the minimum index such that j > i and prices[j] <= prices[i] 矛盾.
     */
    vector<int> finalPrices(vector<int>& prices) {
        vector<int> result = prices;
        stack<int> st;
        for (int i = 0; i < (int)prices.size(); ++i) {
            while (!st.empty()) {
                int index = st.top();
                if (prices[i] <= prices[index]) {
                    result[index] -= prices[i];
                    st.pop();
                } else {
                    break;
                }
            }
            st.push(i);
        }
        return result;
    }
};
