limit = 28123
########## Solution ##########
limit += 1
dsum_list = [0] * limit
for i in range(1, len(dsum_list)):
    for j in range(i * 2, len(dsum_list), i):
        dsum_list[j] += i
    
abundants = [i for i in range(12, limit) if dsum_list[i] > i]
abdsum_list = set(range(1, limit))
for i in abundants:
    for j in abundants:
        if i + j in abdsum_list:
            abdsum_list.remove(i + j)
print(sum(abdsum_list))