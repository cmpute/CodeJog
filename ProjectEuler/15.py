# Solution is C(size, 2*size)
size = 20
########## Solution ##########
sum = 1
for i in range(size+1, 2*size+1):
    sum *= i
for i in range(1,size+1):
    sum //= i
print(sum)
