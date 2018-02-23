limit = 4000000

fib = [1, 1]
total = 1
while(fib[-1] < limit):
    if(fib[-1] & 1):
        total += fib[-1]
    fib.append(fib[-1] + fib[-2])
print(total)