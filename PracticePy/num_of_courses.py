def check_courses(numCourses, prerequisites):
    graph = [[] for _ in range(numCourses)]
    visited = [0] * numCourses

    for course, prereq in prerequisites:
        graph[course].append(prereq)

    def dfs(course):
        if visited[course] == 1:
            return False
        if visited[course] == 2:
            return True

        visited[course] = 1

        for neighbor in graph[course]:
            if not dfs(neighbor):
                return False

        visited[course] = 2
        return True

    for course in range(numCourses):
        if not dfs(course):
            return False

    return True

# Example usage:
numCourses = 4
prerequisites = [[1,0],[2,0],[3,1],[3,2]]
print(check_courses(numCourses, prerequisites))  # Output: True
