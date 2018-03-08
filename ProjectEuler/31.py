# Brute-force recursion
target = 200
coins = (1, 2, 5, 10, 20, 50, 100, 200)
########## Solution ##########
coins = sorted(coins, reverse=True)

def solve(target, index=0):
    if index == len(coins) - 1:
        return target % coins[index] == 0
    return sum(solve(target - coins[index] * i, index + 1)
               for i in range(target // coins[index] + 1))
        
print(solve(target))
