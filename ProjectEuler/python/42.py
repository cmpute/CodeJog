URL = 'https://projecteuler.net/project/resources/p042_words.txt'

from em import sqrt

def check_trianglar(word):
    wsum = 0
    for c in word:
        wsum += ord(c) - 64
    n = sqrt(wsum * 2)
    return n * (n+1) == wsum * 2

def solve():
    from urllib.request import urlopen
    content = urlopen(URL).read().decode('ascii')
    result = [check_trianglar(n.strip('"')) for n in content.split(',')]
    return sum(result)

if __name__ == "__main__":
	print(solve())
