from collections import deque

def mergeSort(array):
    """
    Do merge sort on array[left, right], return the sorted array and
        corresponding counts on that interval

    See sort.py for merge sort template
    """
    if len(array) == 0:
        return [], []
    if len(array) == 1:
        return array, [0]
    mid = len(array) >> 1
    
    larray, lcounts = mergeSort(array[:mid])
    rarray, rcounts = mergeSort(array[mid:])
    
    lidx = ridx = 0
    result, counts = [], []
    while lidx < len(larray) and ridx < len(rarray):
        # sorted in ascending order to prevent when the numbers are equal
        if larray[lidx][1] <= rarray[ridx][1]:
            result.append(larray[lidx])
            counts.append(lcounts[lidx] + ridx) # only ridx numbers on the right are smaller than current left number
            lidx += 1
        else:
            result.append(rarray[ridx])
            counts.append(rcounts[ridx])
            ridx += 1
    while lidx < len(larray):
        result.append(larray[lidx])
        counts.append(lcounts[lidx] + ridx) # all numbers on the right are smaller than current number
        lidx += 1 
    while ridx < len(rarray):
        result.append(rarray[ridx])
        counts.append(rcounts[ridx])
        ridx += 1

    return result, counts

class Solution(object):
            
    def countSmaller(self, nums):
        """
        :type nums: List[int]
        :rtype: List[int]
        """
        nums = list(enumerate(nums))
        ordered, counts = mergeSort(nums)
        result = [0] * len(ordered)
        # FIXME Possible Improvements: avoid remapping
        for idx, (numloc, _) in enumerate(ordered):
            result[numloc] = counts[idx]
        return result
