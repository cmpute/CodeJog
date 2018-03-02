# Brute-force, can be improved by using a estimator for fibnacci numbers
digits = 1000
########## Solution ##########
f1 = 1
f2 = 1
fi = f1 + f2
index = 3
while len(str(fi)) < digits:
    f1, f2, fi = f2, fi, f2 + fi
    index += 1
print(index)
