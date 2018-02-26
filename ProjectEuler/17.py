limit = 1000
########## Solution ##########
nfirst = [4,3,3,5,4,4,3,5,5,4] # 0-9
nteen = [3,6,6,8,8,7,7,9,8,8] # 10-19
nty = [None,None,6,6,5,5,5,7,6,6] # 20-90
nhundred = 7
nthousand = 8
nand = 3

def count_english(n):
    if 0 <= n < 10:
        return nfirst[n]
    if 10 <= n < 20:
        return nteen[n - 10]
    if 20 <= n < 100:
        return nty[n // 10] + (nfirst[n % 10] if n % 10 != 0 else 0)
    if 100 <= n < 1000:
        return nfirst[n // 100] + nhundred + (nand + count_english(n % 100) if n % 100 != 0 else 0)
    if 1000 <= n < 1000000:
        return count_english(n // 1000) + nthousand + (count_english(n % 1000) if n % 1000 != 0 else 0)

print(sum(count_english(i) for i in range(1, limit + 1)))
