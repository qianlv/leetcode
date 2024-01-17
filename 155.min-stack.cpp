/*
 * @lc app=leetcode id=155 lang=cpp
 *
 * [155] Min Stack
 */

// @lc code=start
#include <vector>
class MinStack {
  private:
    std::vector<int> stack;
    std::vector<int> minVals;

  public:
    MinStack() {}

    void push(int val) {
        stack.push_back(val);
        if (minVals.empty() || val <= minVals.back()) {
            minVals.push_back(val);
        }
    }

    void pop() {
        if (stack.back() == minVals.back()) {
            minVals.pop_back();
        }
        stack.pop_back();
    }

    int top() { return stack.back(); }

    int getMin() { return minVals.back(); }
};

/**
 * Your MinStack object will be instantiated and called as such:
 * MinStack* obj = new MinStack();
 * obj->push(val);
 * obj->pop();
 * int param_3 = obj->top();
 * int param_4 = obj->getMin();
 */
// @lc code=end
