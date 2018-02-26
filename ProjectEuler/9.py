sum_target = 1000
########## Solution ##########
from math import sqrt
pairs = []
flag = False
for i in range(sum_target):
    for j in range(3, i):
        k = sum_target - i - j
        if k < i : break
        if i * i + j * j == k * k:
            print(i * j * k)
            flag = True
    if flag:
        break
