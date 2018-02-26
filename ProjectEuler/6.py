# (1+2+...+n)^2=1^3+2^3+...n^3
limit = 100
########## Solution ##########
print(sum((n-1) * n * n for n in range(limit + 1)))
