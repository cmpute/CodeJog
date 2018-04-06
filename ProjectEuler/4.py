def solve():
    '''
    Brute-force solution
    '''
    return max(i * j for i in range(100, 1000) for j in range(100, 1000)
          if str(i * j) == str(i * j)[::-1])

if __name__ == "__main__":
	print(solve())
