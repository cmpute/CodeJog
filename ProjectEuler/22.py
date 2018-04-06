URL = 'https://projecteuler.net/project/resources/p022_names.txt'

def count_word(word):
    wsum = 0
    for c in word:
        wsum += ord(c) - 64
    return wsum

def solve():
    from urllib.request import urlopen
    content = urlopen(URL).read().decode('ascii')
    names = [n.strip('"') for n in content.split(',')]
    total = 0
    for idx, name in enumerate(sorted(names)):
        total += count_word(name) * (idx+1) 
    return total

if __name__ == "__main__":
	print(solve())
