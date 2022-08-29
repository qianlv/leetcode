from typing import List

class Solution:
    def garbageCollection(self, garbage: List[str], travel: List[int]) -> int:
        total_take_garbege = 0
        last_m_index = -1
        last_p_index = -1
        last_g_index = -1
        for i, g in enumerate(garbage):
            total_take_garbege += len(g)
            if 'M' in g:
                last_m_index = i
            if 'P' in g:
                last_p_index = i
            if 'G' in g:
                last_g_index = i

        sum_travel_time = 0
        for i, t in enumerate(travel):
            sum_travel_time += t
            if i + 1 == last_m_index:
                total_take_garbege += sum_travel_time
            if i + 1 == last_p_index:
                total_take_garbege += sum_travel_time
            if i + 1 == last_g_index:
                total_take_garbege += sum_travel_time
        return total_take_garbege


if __name__ == "__main__":
    sol = Solution()
    print(sol.garbageCollection(["G","P","GP","GG"], [2,4,3]))
    print(sol.garbageCollection(garbage = ["MMM","PGM","GP"], travel = [3,10]))

