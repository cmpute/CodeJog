#Use built-in python module
century = 20
########## Solution ##########
start = (century - 1) * 100
end = century * 100
from datetime import date
print(sum(date(year, month, 1).weekday() == 6
          for year in range(start+1, end+1)
          for month in range(1, 13)))
