class Solution:
    def removeStars(self, s: str) -> str:
        st = []
        for ch in s:
            if ch != '*':
                st.append(ch)
            else:
                st.pop()

        return ''.join(st)



if __name__ == "__main__":
    sol = Solution()
    s = sol.removeStars("leet**cod*e")
    print(s)
    assert(s == "lecoe")
    s = sol.removeStars("erase*****")
    assert(s == "")
