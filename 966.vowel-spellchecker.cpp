/*
 * @lc app=leetcode id=966 lang=cpp
 *
 * [966] Vowel Spellchecker
 */

#include <algorithm>
#include <array>
#include <iostream>
#include <string>
#include <vector>

// cspell: disable

// @lc code=start
class Trie {
  private:
    static const int ALPHABET_SIZE = 53;
    struct TrieNode {
        std::string word;
        std::array<TrieNode *, ALPHABET_SIZE> children;
        TrieNode() : children({nullptr}) {}
    };
    TrieNode *root;

    static constexpr int ALPHABET_OFFSET = 26;

    static unsigned char2Index(char character) {
        if (character >= 'a' && character <= 'z') {
            return character - 'a';
        }
        if (character >= 'A' && character <= 'Z') {
            return character - 'A' + ALPHABET_OFFSET;
        }
        return ALPHABET_SIZE - 1;
    }

    void deleteTrie(TrieNode *node) {
        if (node == nullptr) {
            return;
        }
        for (auto *child : node->children) {
            deleteTrie(child);
        }
        delete node;
    }

  public:
    Trie() : root(new TrieNode()) {}
    Trie(const Trie &) = delete;
    Trie &operator=(const Trie &) = delete;
    ~Trie() { deleteTrie(root); }
    void addWord(const std::string &word, const std::string &originalWord) {
        TrieNode *current = root;
        for (auto character : word) {
            const unsigned index = char2Index(character);
            if (current->children.at(index) == nullptr) {
                current->children.at(index) = new TrieNode();
            }
            current = current->children.at(index);
        }
        if (current->word.empty()) {
            current->word = originalWord;
        }
    }

    bool search(const std::string &word, std::string &result) {
        TrieNode *current = root;
        for (auto character : word) {
            const unsigned index = char2Index(character);
            if (current->children.at(index) == nullptr) {
                return false;
            }
            current = current->children.at(index);
        }
        result = current->word;
        return !result.empty();
    }
};
class Solution {
    static constexpr std::string_view VOWELS = "aeiou";

  public:
    static std::vector<std::string> spellchecker(std::vector<std::string> &wordlist,
                                                 std::vector<std::string> &queries) {
        Trie original;
        Trie lower;
        Trie vowel;
        for (auto &word : wordlist) {
            original.addWord(word, word);
            std::string lowerWord = word;
            std::transform(lowerWord.begin(), lowerWord.end(), lowerWord.begin(), ::tolower);
            lower.addWord(lowerWord, word);

            std::string vowelWord = lowerWord;
            for (const auto &vowel : VOWELS) {
                std::replace(vowelWord.begin(), vowelWord.end(), vowel, '*');
            }
            vowel.addWord(vowelWord, word);
        }

        std::vector<std::string> results;

        for (auto &query : queries) {
            std::string result;
            if (original.search(query, result)) {
                results.push_back(result);
                continue;
            }

            std::string lowerQuery = query;
            std::transform(lowerQuery.begin(), lowerQuery.end(), lowerQuery.begin(), ::tolower);
            if (lower.search(lowerQuery, result)) {
                results.push_back(result);
                continue;
            }
            std::string vowelQuery = lowerQuery;
            for (const auto &vowel : VOWELS) {
                std::replace(vowelQuery.begin(), vowelQuery.end(), vowel, '*');
            }
            if (vowel.search(vowelQuery, result)) {
                results.push_back(result);
                continue;
            }
            results.emplace_back();
        }
        return results;
    }
};
// @lc code=end

int main(int argc, const char **argv) {
    std::vector<std::string> wordlist = {"KiTe", "kite", "hare", "Hare"};
    std::vector<std::string> queries = {"kite", "Kite", "KiTe", "Hare", "HARE",
                                        "Hear", "hear", "keti", "keet", "keto"};
    auto results = Solution::spellchecker(wordlist, queries);
    for (auto &result : results) {
        std::cout << result << "\n";
    }
}