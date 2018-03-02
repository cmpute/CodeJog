target = 1001
########## Solution ##########
current = 1
diff = 2
dsum = 1
while diff < target:
    # c, c+d, c+2d, c+3d, c+4d
    dsum += 4 * current + 10 * diff
    current += 4 * diff
    diff += 2
print(dsum)
