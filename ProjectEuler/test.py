import importlib, time

def main():
	totaltime = 0.0  # seconds
	for (idx, expect) in sorted(ANSWERS.items()):
		module = importlib.import_module(str(idx))
		starttime = time.time()
		result = str(module.solve())
		elapsedtime = time.time() - starttime
		totaltime += elapsedtime
		print("Problem {:3d}: {:9.2f} ms     {}".format(
			idx, elapsedtime * 1000,
			"PASS" if result == expect else "FAIL"))
	print("Total computation time: {} ms".format(totaltime))

ANSWERS = {
	  1: "233168",
	  2: "4613732",
	  3: "6857",
	  4: "906609",
	  5: "232792560",
	  6: "25164150",
	  7: "104743",
	  8: "23514624000",
	  9: "31875000",
	 10: "142913828922",
	 11: "70600674",
	 12: "76576500",
	 13: "5537376230",
	 14: "837799",
	 15: "137846528820",
	 16: "1366",
	 17: "21124",
	 18: "1074",
	 19: "171",
	 20: "648",
	 21: "31626",
	 22: "871198282",
	 23: "4179871",
	 24: "2783915460",
	 25: "4782",
	 26: "983",
	 27: "-59231",
	 28: "669171001",
	 29: "9183",
	 30: "443839",
	 31: "73682",
	 32: "45228",
	 33: "100",
	 34: "40730",
	 35: "55",
	 36: "872187",
	 37: "748317",
	 38: "932718654",
	 39: "840",
	 40: "210",
	 41: "7652413",
	 42: "162",
	 43: "16695334890",
	 44: "5482660",
	 45: "1533776805",
}

if __name__ == "__main__":
	main()
