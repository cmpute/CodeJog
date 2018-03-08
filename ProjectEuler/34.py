# brute-force
########## Solution ##########
factorial = {'0': 1}
product = 1
for i in range(1,10):
    product *= i
    factorial[str(i)] = product
# maximum n for which n*9! > 10^n is 6
total = 0
for i in range(3, 10**6):
    if sum(factorial[digit] for digit in str(i)) == i:
        total += i
print(total)
