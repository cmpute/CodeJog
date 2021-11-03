URL = 'https://projecteuler.net/project/resources/p059_cipher.txt'

def checki(values, k):
    '''
    The text cannot be perfectly decrypted, so use a score to represent
    '''
    score = 0
    for i, c in enumerate(values):
        decrypt = c ^ k[i % len(k)]
        if ord('a') <= decrypt and decrypt <= ord('z'):
            score += 3
        if ord('A') <= decrypt and decrypt <= ord('Z'):
            score += 2
        if ord('0') <= decrypt and decrypt <= ord('9'):
            score += 1
        if decrypt in [ord(' '), ord('&'), ord('!')]:
            score += 1
    return score

def solve():
    from urllib.request import urlopen
    content = urlopen(URL).read().decode('ascii')
    content = [int(n) for n in content.split(',')]
    scores = {(k0, k1, k2): checki(content, [k0,k1,k2])
        for k0 in range(ord('a'), ord('z')+1)
        for k1 in range(ord('a'), ord('z')+1)
        for k2 in range(ord('a'), ord('z')+1)
    }
    bkey = max(scores, key=scores.get)

    total = 0
    for i, c in enumerate(content):
        total += c ^ bkey[i % len(bkey)]
    return total

if __name__ == "__main__":
    print(solve())
