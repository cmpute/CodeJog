# Brute-force
exp = 5
########## Solution ##########
powdict = dict()
maxlen = 1
total = 0
for i in range(10):
    powdict[str(i)] = i ** exp
while int(''.join(['1'] + (maxlen - 1)*['9'])) < maxlen * powdict['9']:
    maxlen += 1
for i in range(2, maxlen * powdict['9']):
    if sum(powdict[d] for d in str(i)) == i:
        total += i
print(total)
