limit = 1000
########## Solution ##########
from em import sqrt
limit //= 2
result = dict()
for i in range(3, limit):
    for j in range(3, limit - i):
        ks = i*i + j*j
        k = sqrt(ks)
        if k*k == ks:
            perimeter = i + j + k
            if perimeter in result:
                result[perimeter] += 1
            else:
                result[perimeter] = 1
print(max(result, key=result.get))
