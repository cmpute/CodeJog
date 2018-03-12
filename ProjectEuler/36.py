# brute-force
limit = 1000000
########## Solution ##########
print(sum(i for i in range(limit) if str(i) == str(i)[::-1] and bin(i)[2::] == bin(i)[:1:-1]))
