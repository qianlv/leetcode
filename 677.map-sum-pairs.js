/*
 * @lc app=leetcode id=677 lang=javascript
 *
 * [677] Map Sum Pairs
 */

// @lc code=start

function TrieNode() {
  const ALPHA_SIZE = 26;
  this.children = new Array(ALPHA_SIZE).fill(null);
  this.score = 0;
  this.sum = 0;
}


TrieNode.prototype.get = function (char) {
  return this.children[char.charCodeAt(0) - 'a'.charCodeAt(0)];
}

TrieNode.prototype.put = function (char, node) {
  this.children[char.charCodeAt(0) - 'a'.charCodeAt(0)] = node;
}

var MapSum = function () {
  this.root = new TrieNode();
};

/** 
 * @param {string} key 
 * @param {number} val
 * @return {void}
 */
MapSum.prototype.insert = function (key, val) {
  let current = this.root;
  for (const char of key) {
    if (!current.get(char)) {
      current.put(char, new TrieNode());
    }
    current = current.get(char);
  }
  const updateScore = val - current.score;
  current.score = val;

  current = this.root;
  for (const char of key) {
    current = current.get(char);
    if (!current) {
      break;
    }
    current.sum += updateScore;
  }
};

/** 
 * @param {string} prefix
 * @return {number}
 */
MapSum.prototype.sum = function (prefix) {
  let current = this.root;
  for (const char of prefix) {
    if (!current.get(char)) {
      return 0;
    }
    current = current.get(char);
  }
  return current.sum;
};

/**
 * Your MapSum object will be instantiated and called as such:
 * var obj = new MapSum()
 * obj.insert(key,val)
 * var param_2 = obj.sum(prefix)
 */
// @lc code=end

let obj = new MapSum()
obj.insert("apple", 3);
console.log(obj.sum("ap"));

