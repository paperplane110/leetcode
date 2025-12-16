from typing import List


def wordBreak(s: str, wordDict: List[str]) -> bool:
    def search(s, wordDict):
        if s in false_hist:
            return False
        for word in wordDict:
            if s == word:
                return True
            if s.startswith(word):
                new_s = s[len(word)::]
                if search(new_s, wordDict):
                    return True
        false_hist.add(s)
        return False

    new_word_list = wordDict.copy()
    false_hist = set()
    for i in range(len(new_word_list)):
        if search(new_word_list[i], new_word_list[0:i]+new_word_list[i+1::]):
            print(new_word_list[i])
            wordDict.remove(new_word_list[i])
    print(wordDict)
    false_hist = set()
    return search(s, wordDict)


s = "leetcode"
wordDict = ["leet", "code"]


s = "catsandog"
wordDict = ["cats", "dog", "sand", "and", "cat"]

s = "bb"
wordDict = ["a", "b", "bbb", "bbbb"]

s = "bccdbacdbdacddabbaaaadababadad"
wordDict = ["cbc", "bcda", "adb", "ddca", "bad", "bbb", "dad", "dac", "ba", "aa", "bd", "abab", "bb", "dbda", "cb", "caccc", "d", "dd", "aadb", "cc", "b", "bcc", "bcd", "cd", "cbca", "bbd",
            "ddd", "dabb", "ab", "acd", "a", "bbcc", "cdcbd", "cada", "dbca", "ac", "abacd", "cba", "cdb", "dbac", "aada", "cdcda", "cdc", "dbc", "dbcb", "bdb", "ddbdd", "cadaa", "ddbc", "babb"]
print(wordBreak(s, wordDict))
