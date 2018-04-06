CENTURY = 20

def solve(century = CENTURY):
    '''
    Use built-in python module
    '''
    start = (century - 1) * 100
    end = century * 100
    from datetime import date
    return sum(date(year, month, 1).weekday() == 6
            for year in range(start+1, end+1)
            for month in range(1, 13))

if __name__ == "__main__":
	print(solve())
