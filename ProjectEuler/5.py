limit = 20
########## Solution ##########
def gcd(a, b):
    if(b > a):
        return gcd(b, a)
    if(a % b == 0):
        return b
    return gcd(b, a % b)

total = 1
for i in range(2, limit + 1):
    total = total * i // gcd(total, i)
print(total)
