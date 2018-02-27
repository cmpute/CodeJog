url = 'https://projecteuler.net/project/resources/p022_names.txt'
########## Solution ##########
def count_word(word):
    wsum = 0
    for c in word:
        wsum += ord(c) - 64
    return wsum

from urllib.request import urlopen
content = urlopen(url).read().decode('ascii')
names = [n.strip('"') for n in content.split(',')]
total = 0
for idx, name in enumerate(sorted(names)):
    total += count_word(name) * (idx+1) 
print(total)
