DIGITS = 1000

def solve(digits = DIGITS):
    '''
    Brute-force solution, can be improved by using a estimator for fibnacci numbers
    '''
    f1 = 1
    f2 = 1
    fi = f1 + f2
    index = 3
    while len(str(fi)) < digits:
        f1, f2, fi = f2, fi, f2 + fi
        index += 1
    return index

if __name__ == "__main__":
	print(solve())
