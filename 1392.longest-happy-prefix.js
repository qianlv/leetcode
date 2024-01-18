/*
 * @lc app=leetcode id=1392 lang=javascript
 *
 * [1392] Longest Happy Prefix
 */

// @lc code=start

/**
 * @param {string} s
 * @return {string}
 */
var longestPrefix = function (s) {
  function kmp() {
    let next = new Array(s.length).fill(0);

    for (let i = 1; i < s.length; i++) {
      let j = next[i - 1];
      while (j > 0 && s[i] !== s[j]) {
        j = next[j - 1];
      }
      next[i] = j + (s[i] === s[j] ? 1 : 0);
    }
    return next;
  }

  const next = kmp();
  return s.slice(0, next[s.length - 1]);
};
// @lc code=end

