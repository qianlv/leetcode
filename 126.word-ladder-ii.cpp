/*
 * @lc app=leetcode id=126 lang=cpp
 *
 * [126] Word Ladder II
 */

// @lc code=start
#include <climits>
#include <queue>
#include <string>
#include <unordered_map>
#include <vector>

using std::string;
using std::unordered_map;
using std::vector;

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

    void dfs(const std::vector<std::string> &wordList, int beginIndex, int endIndex,
             std::vector<std::vector<std::string>> &result, std::vector<std::string> &path,
             std::unordered_map<int, std::vector<int>> &wordMap) {
        if (beginIndex == endIndex) {
            std::vector<std::string> tmp(path.rbegin(), path.rend());
            result.push_back(tmp);
            return;
        }

        for (size_t i = 0; i < wordMap[endIndex].size(); i++) {
            int index = wordMap[endIndex][i];
            path.push_back(wordList[index]);
            dfs(wordList, beginIndex, index, result, path, wordMap);
            path.pop_back();
        }
    }

    void debugGraph() {
        for (size_t i = 0; i < graph.size(); i++) {
            printf("%ld: ", i);
            for (size_t j = 0; j < graph[i].size(); j++) {
                printf("%d ", graph[i][j]);
            }
            printf("\n");
        }
    }

  public:
    vector<vector<string>> findLadders(string beginWord, string endWord, vector<string> &wordList) {
        int endIndex = -1;
        for (size_t i = 0; i < wordList.size(); i++) {
            if (wordList[i] == endWord) {
                endIndex = i;
                break;
            }
        }
        if (endIndex == -1) {
            return {};
        }

        int beginIndex = wordList.size();
        // printf("beginIndex: %d, endIndex: %d\n", beginIndex, endIndex);
        wordList.push_back(beginWord);
        initGraph(wordList);
        // debugGraph();

        // bfs
        vector<vector<string>> result;
        unordered_map<int, std::vector<int>> wordMap;
        std::vector<int> distance(wordList.size(), INT_MAX);
        std::queue<int> que;
        que.push(beginIndex);
        wordMap[beginIndex] = {};
        distance[beginIndex] = 0;
        while (!que.empty()) {
            int from = que.front();
            que.pop();
            // printf("%d: ", from);

            for (int to : graph[from]) {
                if (wordMap.find(to) == wordMap.end()) {
                    // printf("%d\n", to);
                    que.push(to);
                    // printf("to: %d, from: %d\n", to, from);
                    wordMap[to].push_back(from);
                    distance[to] = distance[from] + 1;
                } else if (distance[to] == distance[from] + 1) {
                    // printf("to: %d, from: %d\n", to, from);
                    wordMap[to].push_back(from);
                }
            }
        }

        if (wordMap.find(endIndex) == wordMap.end()) {
            return result;
        }

        std::vector<std::string> path;
        dfs(wordList, beginIndex, endIndex, result, path, wordMap);
        for (auto &path : result) {
            path.push_back(endWord);
        }

        return result;
    }
};

// int main(int argc, const char **argv) {
//   Solution solution;
//   std::vector<std::string> wordList = {"hot", "dot", "dog",
//                                        "lot", "log", "cog"};
//   // std::vector<std::string> wordList = {"hot", "dot", "dog", "lot", "log"};
//   auto result = solution.findLadders("hit", "cog", wordList);
//   printf("result size: %ld\n", result.size());
//   for (auto &path : result) {
//     for (auto &word : path) {
//       printf("%s ", word.c_str());
//     }
//     printf("\n");
//   }
// }
// @lc code=end
