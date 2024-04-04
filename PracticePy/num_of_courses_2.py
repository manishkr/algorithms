from collections import defaultdict, deque

def find_order(numCourses, prerequisites):
    # Create adjacency list representation of the graph
    graph = defaultdict(list)
    in_degree = [0] * numCourses

    for course, prereq in prerequisites:
        graph[prereq].append(course)
        in_degree[course] += 1

    # Initialize a queue with courses having in-degree 0
    queue = deque([course for course in range(numCourses) if in_degree[course] == 0])

    # Initialize the result list
    result = []

    # Perform topological sorting
    while queue:
        course = queue.popleft()
        result.append(course)

        # Decrease in-degree of adjacent courses
        for neighbor in graph[course]:
            in_degree[neighbor] -= 1
            if in_degree[neighbor] == 0:
                queue.append(neighbor)

    # If there are still courses left, it means there's a cycle
    if len(result) != numCourses:
        return False

    return True

if __name__ == '__main__':
    courses = [[1,0]]
    num_courses =2
    result = find_order(num_courses, courses)
    print(result)
