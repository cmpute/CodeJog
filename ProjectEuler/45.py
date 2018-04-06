# Every hexagonal number is also a triangular number
########## Solution ##########
from em import iterpolygonal

def join():
    penta = iterpolygonal(5)
    hexa = iterpolygonal(6)
    cpenta = -1
    chexa = -2
    while True:
        if cpenta < chexa:
            cpenta = next(penta)
        elif chexa < cpenta:
            chexa = next(hexa)
        else:
            yield cpenta
            cpenta += 1

result = join()
next(result) # 1
next(result) # number in the problem
print(next(result))
