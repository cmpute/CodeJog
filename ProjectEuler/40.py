# Simulation
targets = [1, 10, 100, 1000, 10000, 100000, 1000000]
########## Solution ##########
def generate():
    current = 1
    while True:
        for d in str(current):
            yield d
        current += 1

product = 1
for idx, d in enumerate(generate()):
    if idx + 1 in targets:
        product *= int(d)
    if idx > targets[-1]:
        break
print(product)
