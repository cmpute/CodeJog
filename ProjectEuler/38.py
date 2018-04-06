def solve():
    result = ''
    for n in range(2, 6): # n <= 5
        for i in range(1, 10**(9//n)):
            concatenated_product = ''.join(str(i * j) for j in range(1, n+1))
            if ''.join(sorted(concatenated_product)) == '123456789':
                result = max(result, concatenated_product)
    return result

if __name__ == "__main__":
	print(solve())
