"""
This module contains clean algorithm templates for easy deep modification.
"""

def merge_sort(array: list) -> list:
    """
    Merge sort template for ascending sorting

    TODO: Advances versions
    1. Use linked list to boost merging process
    2. Use bottom-up strategy to avoid recursion (like radix sort)
    """
    # terminate condition for recursion
    if len(array) <= 1:
        return array
    mid = len(array) >> 1
    
    # divide into subproblems
    larray = merge_sort(array[:mid])
    rarray = merge_sort(array[mid:])
    
    # merge sorted subarrays
    lidx = ridx = 0
    result = []
    while lidx < len(larray) and ridx < len(rarray):
        if larray[lidx] <= rarray[ridx]: # ensure stable sorting by using "<="
            result.append(larray[lidx])
            lidx += 1
        else:
            result.append(rarray[ridx])
            ridx += 1
    while lidx < len(larray): # ensure stable sorting by merging left part first
        result.append(larray[lidx])
        lidx += 1 
    while ridx < len(rarray):
        result.append(rarray[ridx])
        ridx += 1

    return result

def quick_sort(array: list) -> list:
    """
    Merge sort template for ascending sorting
    """
    raise NotImplementedError()
