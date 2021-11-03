TARGET = 200
COINS = (1, 2, 5, 10, 20, 50, 100, 200)

def solve(target = TARGET, coins = COINS):
    '''
    Brute-force recursion
    '''
    coins = sorted(coins, reverse=True)

    def sub(target, index=0):
        if index == len(coins) - 1:
            return target % coins[index] == 0
        return sum(sub(target - coins[index] * i, index + 1)
                for i in range(target // coins[index] + 1))
            
    return sub(target)

if __name__ == "__main__":
	print(solve())
