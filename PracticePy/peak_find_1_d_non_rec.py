def find_peak(nums):
    if len(nums) == 0:
        return -1
    if len(nums) == 1:
        return 0
    if nums[0] > nums[1]:
        return 0
    if nums[-1] > nums[-2]:
        return len(nums) - 1

    l = 0
    r = len(nums) - 1
    while l <= r:
        mid = r - (r - l)/2
        if nums[mid-1] > nums[mid]:
            r = mid -1
        elif nums[mid] < nums[mid + 1]:
            l = mid + 1
        else:
            return mid

if __name__ == '__main__':
    l = [1,1,1,2,3,4,6]
    result = find_peak(l)
    print(result)
