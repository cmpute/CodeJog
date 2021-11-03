MAXITER = 50
LIMIT = 10000

def isPalindromic(number):
    snum = str(number)
    return snum == snum[::-1]

def isLychrel(number, maxiter):
    # XXX: To improve efficiency, lychrel path can be cached in a set
    counter = maxiter
    while counter > 0:
        number += int(str(number)[::-1])
        if isPalindromic(number):
            return True
        counter -= 1
    return False

def solve(maxiter = MAXITER, limit = LIMIT):
    return sum(not isLychrel(i, maxiter) for i in range(limit))

if __name__ == "__main__":
	print(solve())
