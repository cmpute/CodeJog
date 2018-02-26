limit = 1000
########## Solution ##########
mark = set()
three = five = total = 0
while three < limit:
    total += three
    mark.add(three)
    three += 3
while five < limit:
    if(five not in mark):
        total += five
    five += 5
print(total)
