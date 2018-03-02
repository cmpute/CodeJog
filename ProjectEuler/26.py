# Reference: 薛利敏.纯循环小数循环节的规律[J].渭南师范学院学报,2001(04):92-94.
limit = 1000
########## Solution ##########
from em import gcd, mulmod
def get_cycle_length(target):
    if gcd(10, target) > 1:
        return 0
    counter = 1
    reminder = mulmod(10, counter, target)
    while counter < target:
        if reminder == 1:
            return counter
        reminder = mulmod(10, reminder, target)
        counter += 1
    raise Exception('Cannot get the cycle length of ' + str(target))

print(max(range(2, limit), key=get_cycle_length))
