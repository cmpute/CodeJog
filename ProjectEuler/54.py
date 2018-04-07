URL = "https://projecteuler.net/project/resources/p054_poker.txt"

ranks = "23456789TJQKA"
suits = "SHCD"

def score(cards):
    rankcounts = tuple(sum(ranks.find(rank) == i for rank, _ in cards) for i in range(13))
    rankhist = tuple(rankcounts.count(i) for i in range(6))
    
    suitlist = tuple(suits.find(suit) for _, suit in cards)
    minsuit = min(suitlist)
    maxsuit = max(suitlist)
    
    # encode cards in descending order into 20 bits
    encode = 0
    for i in reversed(range(len(rankhist))):
        for j in reversed(range(len(rankcounts))):
            if rankcounts[j] == i:
                for _ in range(i):
                    encode = encode << 4 | j

    # check straight
    straight = False
    for i in reversed(range(3, len(rankcounts))):
        for j in range(5):
            if rankcounts[(i - j + 13) % 13] == 0:
                break
        else:
            straight = i
            break
    
    # compose score
    if   straight and minsuit != maxsuit        : return 8 << 20 | straight # Straight flush, including Royal Flush
    elif rankhist[4] == 1                       : return 7 << 20 | encode   # Four of a kind
    elif rankhist[3] == 1 and rankhist[2] == 1  : return 6 << 20 | encode   # Full house
    elif minsuit == maxsuit                     : return 5 << 20 | encode   # Flush
    elif straight                               : return 4 << 20 | straight # Straight
    elif rankhist[3] == 1                       : return 3 << 20 | encode   # Three of a kind
    elif rankhist[2] == 2                       : return 2 << 20 | encode   # Two pairs
    elif rankhist[2] == 1                       : return 1 << 20 | encode   # One pair
    else                                        : return 0 << 20 | encode   # High card

def solve():
    from urllib.request import urlopen
    content = urlopen(URL).read().decode('ascii')

    total = 0
    for line in content.split('\n'):
        if len(line) < 20: continue
        cards = line.split(' ')
        if score(cards[:5]) > score(cards[5:]):
            total += 1
    return total

if __name__ == "__main__":
    print(solve())
