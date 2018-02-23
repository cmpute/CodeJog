limit = 20

total = 1
def gcd(a, b):
    if(b > a):
        return gcd(b, a)
    if(a % b == 0):
        return b
    return gcd(b, a % b)
for i in range(2, limit + 1):
    total = total * i // gcd(total, i)
print(total)
