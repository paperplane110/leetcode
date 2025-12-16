label = "今天天气很好，很适合出游。十一的天气怎么样，你有什么安排？我打算出游，去周边走走。"
pre = "今天天气很好，很适合出游。十一的天气,怎么样？你有什么安排？我打算出游去周边走走。"

PUNCTUATION = ['，', '。', '？']


def eval(gt, pre):
    i, j = 0, 0

    tp, tn, fp, fn = 0, 0, 0, 0
    while i < len(gt):
        if gt[i] in PUNCTUATION:
            if gt[i] == pre[j]:
                # both punctuation and same
                tp += 1
                i += 1
                j += 1

            elif pre[j] in PUNCTUATION:
                # different punctuation
                fp += 1
                i += 1
                j += 1

            else:
                # missing punctuation
                fn += 1
                i += 1

        else:
            if gt[i] == pre[j]:
                # same character
                tn += 1
                i += 1
                j += 1

            else:
                # redundant punctuation
                fp += 1
                j += 1

    print('precision', tp/(tp+fp))
    print('recall', tp/(tp+fn))


eval(label, pre)
