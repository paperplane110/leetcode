"""
给你一个字符串数组，请你将 字母异位词 组合在一起。可以按任意顺序返回结果列表。

字母异位词 是由重新排列源单词的字母得到的一个新单词，所有源单词中的字母通常恰好只用一次。

输入: strs = ["eat", "tea", "tan", "ate", "nat", "bat"]
输出: [["bat"],["nat","tan"],["ate","eat","tea"]]
"""
from typing import List


def groupAnagrams(strs: List[str]) -> List[List[str]]:
    hashmap: dict[str, list] = {}
    for s in strs:
        tmp = ''.join(sorted(s))
        if tmp not in hashmap:
            hashmap[tmp] = [s]
        else:
            hashmap[tmp].append(s)

    ans = []
    for k, group in hashmap.items():
        ans.append(group)

    return ans


strs = [""]
print(groupAnagrams(strs))
