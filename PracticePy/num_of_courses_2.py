from collections import defaultdict, deque
def find_order(num_courses, courses):
    course_map = defaultdict(list)
    in_degree = [0]*num_courses
    for course_pair in courses:
        course_map[course_pair[1]].append(course_pair[0])
        in_degree[course_pair[0]] +=1

    print(in_degree)
    queue = deque([course for course in range(num_courses) if in_degree[course] == 0])
    result = []

    while queue:
        course = queue.popleft()
        result.append(course)

        for neighbour in course_map[course]:
            in_degree[neighbour] -=1
            if in_degree[neighbour] == 0:
                queue.append(neighbour)

    if len(result) != num_courses:
        return []

    return result

if __name__ == '__main__':
    courses = [[1,0]]
    num_courses =2
    result = find_order(num_courses, courses)
    print(result)

    courses =[[1,0],[2,0],[3,1],[3,2]]
    num_courses =4
    result = find_order(num_courses, courses)
    print(result)
