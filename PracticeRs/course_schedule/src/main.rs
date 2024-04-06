use std::collections::VecDeque;
fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    let mut course_map: Vec<Vec<i32>> = Vec::new();
    let mut in_degree: Vec<i32> = Vec::new();
    for _ in 0..num_courses {
        course_map.push(Vec::new());
        in_degree.push(0);
    }
    for course_pair in prerequisites.iter() {
        course_map[course_pair[1] as usize].push(course_pair[0]);
        in_degree[course_pair[0] as usize] += 1;
    }
    let mut queue: VecDeque<i32> = VecDeque::new();
    for course in 0..in_degree.len() {
        if in_degree[course] == 0 {
            queue.push_back(course as i32);
        }
    }

    let mut result = Vec::new();
    while queue.len() > 0 {
        if let Some(course) = queue.pop_back() {
            result.push(course);

            for neighbour in course_map[course as usize].iter() {
                in_degree[*neighbour as usize] -= 1;
                if in_degree[*neighbour as usize] == 0 {
                    queue.push_back(*neighbour)
                }
            }
        }
    }
    if result.len() != num_courses as usize {
        return vec![];
    }
    result
}

fn main() {
    let courses = vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]];
    println!("{:?}", find_order(4, courses));
    let courses = vec![vec![1, 0]];
    println!("{:?}", find_order(2, courses));
}
