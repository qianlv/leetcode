/*
 * @lc app=leetcode id=3008 lang=cpp
 *
 * [3008] Find Beautiful Indices in the Given Array II
 */
#include <iostream>
#include <string>
#include <vector>

using namespace std;

// @lc code=start
class Solution {
    static const unsigned POW = 113;

  private:
    vector<unsigned> hashS;
    vector<unsigned> pows;
    unsigned hashA;
    unsigned hashB;

  public:
    vector<int> beautifulIndices(string s, string a, string b, int k) {
        hash(s);
        hashA = hashString(a);
        hashB = hashString(b);
        vector<int> aIndices = oneStringIndices(s, a, hashA);
        vector<int> bindices = oneStringIndices(s, b, hashB);
        if (aIndices.empty() || bindices.empty()) {
            return {};
        }
        vector<int> res;
        for (int i : aIndices) {
            auto it = std::lower_bound(bindices.begin(), bindices.end(), i);
            // cout << "i: " << i << " it: " << *it << endl;
            if (it != bindices.end()) {
                if (*it - i <= k) {
                    res.push_back(i);
                } else if (it != bindices.begin()) {
                    it--;
                    if (i - *it <= k) {
                        res.push_back(i);
                    }
                }
            } else { // it == bindices.end()
                it--;
                if (i - *it <= k) {
                    res.push_back(i);
                }
            }
        }
        return res;
    }

    vector<int> oneStringIndices(const string &s, const string &a, unsigned hashVal) {
        vector<int> res;
        for (size_t i = 0; i < s.size(); i++) {
            if (i + a.size() > s.size()) {
                break;
            }
            int l = i + 1;
            int r = l + a.size();
            if (s[i] != a[0]) {
                continue;
            }
            if (getHash(l, r) == hashVal) {
                // if (std::equal(a.begin(), a.end(), s.begin() + i)) {
                //     res.push_back(i);
                // }
                // skip the above equal function, since it is not efficient, but the hash is
                // may be the hash value is the same, but the string is different.
                // so use kmp should be better.
                res.push_back(i);
            }
        }

        return res;
    }

    void hash(const string &s) {
        hashS.resize(s.size() + 1, 0);
        pows.resize(s.size() + 1, 0);
        hashS[0] = 0;
        pows[0] = 1;
        for (size_t i = 0; i < s.size(); i++) {
            hashS[i + 1] = hashS[i] * POW + s[i];
            pows[i + 1] = pows[i] * POW;
        }
    }

    unsigned hashString(const string &s) {
        unsigned hashVal = 0;
        for (size_t i = 0; i < s.size(); i++) {
            hashVal = hashVal * POW + s[i];
        }
        return hashVal;
    }

    unsigned getHash(int l, int r) { return hashS[r - 1] - hashS[l - 1] * pows[r - l]; }
};
// @lc code=end

int main(int argc, const char **argv) {
    Solution sol;
    vector<int> res =
        sol.beautifulIndices("isawsquirrelnearmysquirrelhouseohmy", "my", "squirrel", 15);
    for (auto val : res) {
        cout << val << " ";
    }
    cout << '\n';

    res = sol.beautifulIndices("vatevavakz", "va", "lbda", 15);
    for (auto val : res) {
        cout << val << " ";
    }
    cout << '\n';
}