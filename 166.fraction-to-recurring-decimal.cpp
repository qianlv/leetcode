/*
 * @lc app=leetcode id=166 lang=cpp
 *
 * [166] Fraction to Recurring Decimal
 */

// @lc code=start
#include <cstdint>
#include <string>
#include <unordered_map>

class Solution {
public:
  std::string fractionToDecimal(int numerator, int denominator) {
    if (numerator == 0) {
      return "0";
    }

    std::unordered_map<int, int> left;
    std::string ans;
    int i = 0;

    bool sign = (numerator < 0) ^ (denominator < 0);
    int64_t n = numerator < 0 ? -(int64_t)numerator : numerator;
    int64_t d = denominator < 0 ? -(int64_t)denominator : denominator;

    int64_t int_part = 0;
    if (n >= d) {
      int_part = n / d;
      n = n % d;
    }

    n *= 10;
    left[n] = i;
    while (n > 0) {
      int64_t q = n / d;
      ans.push_back(q + '0');
      n = (n % d) * 10;
      if (left.find(n) != left.end()) {
        break;
      }
      left[n] = ++i;
    }

    std::string last_ans;
    last_ans.reserve(ans.size() + 2);
    if (sign) {
      last_ans.push_back('-');
    }
    last_ans.append(std::to_string(int_part));
    if (n == 0) {
      if (!ans.empty()) {
        last_ans.push_back('.');
        last_ans.append(ans);
      }
    } else {
      int loop_start = left[n];
      last_ans.push_back('.');
      for (int j = 0; j < loop_start; ++j) {
        last_ans.push_back(ans[j]);
      }
      last_ans.push_back('(');
      last_ans.append(ans.substr(loop_start));
      last_ans.push_back(')');
    }

    return last_ans;
  }
};

// int main(int argc, const char **argv) {
//   Solution sol;
//   std::string ans = sol.fractionToDecimal(-2147483648, 1);
//   std::cout << ans << std::endl;
// }
// @lc code=end
