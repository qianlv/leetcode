/*
 * @lc app=leetcode id=127 lang=cpp
 *
 * [127] Word Ladder
 */

#include <climits>
#include <queue>
#include <string>
#include <unordered_map>
#include <vector>

using std::string;
using std::unordered_map;
using std::vector;

// @lc code=start
class Solution {
private:
  std::vector<std::vector<int>> graph;

  bool isConnected(const std::string &a, const std::string &b) {
    int diff = 0;
    for (size_t i = 0; i < a.size(); i++) {
      if (a[i] != b[i]) {
        diff++;
      }
    }
    return diff == 1;
  }

  void addEdge(int from, int to) {
    graph[from].push_back(to);
    graph[to].push_back(from);
  }

  void initGraph(const std::vector<std::string> &wordList) {
    graph.resize(wordList.size());
    for (size_t i = 0; i < wordList.size(); i++) {
      for (size_t j = i + 1; j < wordList.size(); j++) {
        if (isConnected(wordList[i], wordList[j])) {
          addEdge(i, j);
        }
      }
    }
  }

public:
  int ladderLength(string beginWord, string endWord, vector<string> &wordList) {
    int endIndex = -1;
    for (size_t i = 0; i < wordList.size(); i++) {
      if (wordList[i] == endWord) {
        endIndex = i;
        break;
      }
    }
    if (endIndex == -1) {
      return 0;
    }

    int beginIndex = wordList.size();
    wordList.push_back(beginWord);
    initGraph(wordList);

    // bfs
    std::vector<int> distance(wordList.size(), INT_MAX);
    std::queue<int> que;
    que.push(beginIndex);
    distance[beginIndex] = 0;
    while (!que.empty()) {
      int from = que.front();
      que.pop();
      if (from == endIndex) {
        break;
      }

      for (int to : graph[from]) {
        if (distance[to] == INT_MAX) {
          que.push(to);
          distance[to] = distance[from] + 1;
        }
      }
    }
    return distance[endIndex] == INT_MAX ? 0 : distance[endIndex] + 1;
  }
};
// @lc code=end
