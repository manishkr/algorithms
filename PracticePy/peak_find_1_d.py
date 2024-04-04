def find_peak_rec(nums, start, end):
    mid = end - (end - start)/2
    if nums[mid-1] > nums[mid]:
        return find_peak_rec(nums, start, mid-1)
    elif nums[mid] < nums[mid + 1]:
        return find_peak_rec(nums, mid+1,end)
    else:
        return mid

def find_peak(nums):
    if len(nums) == 0:
        return -1
    if len(nums) == 1:
        return 0
    if nums[0] > nums[1]:
        return 0
    if nums[-1] > nums[-2]:
        return len(nums) - 1

    return find_peak_rec(nums, 0, len(nums) -1)

if __name__ == '__main__':
    l = [1,1,1,2,3,4,6]
    result = find_peak(l)
    print(result)
