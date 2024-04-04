import copy

def find_all_subset(s):
    subsets = list()
    subsets.append(set())
    for item in s:
        temp_sets = list()
        for subset in subsets:
            copy_subset = copy.deepcopy(subset)
            copy_subset.add(item)
            temp_sets.append(copy_subset)
        subsets.extend(temp_sets)

    return subsets

if __name__ == '__main__':
    s = set([11, 2, 13, 5])
    subsets = find_all_subset(s)
    print(len(subsets))
