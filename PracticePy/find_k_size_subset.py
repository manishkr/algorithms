import copy

def generate_subsets(set_l, k, start, current_subset, result):
    if len(current_subset) == k:
        result.append(copy.deepcopy(current_subset))

    for i in range(start, len(set_l)):
        current_subset.add(set_l[i])
        generate_subsets(set_l, k, i+1, current_subset, result)
        current_subset.remove(set_l[i])

def find_k_size_subsets(s, k):
    result = list()
    current_subset = set()
    set_l = list(s)
    generate_subsets(set_l, k, 0, current_subset, result)
    return result


if __name__ == '__main__':
    s = {11, 2, 1, 3}
    result = find_k_size_subsets(s, 3)
    print(result)
