use tokio::task;

async fn background_task() {
    // Your background task logic goes here
    println!("Background task started");
    let nums = vec![1, 2, 3, 4, 5, 6, 7];
    let sum_value = some_of_nums(&nums);
    println!("{}", sum_value);
    // Simulate some work
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    println!("Background task completed");
}

fn start_background_task() {
    task::spawn(background_task());
    // Function returns immediately without waiting for the background task to complete
}

fn some_of_nums(nums: &[i32]) -> i32 {
    nums.into_iter().sum()
}

#[tokio::main]
async fn main() {
    // Start the background task
    start_background_task();
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    // Continue with other operations
    println!("Main function continues executing");
    // Wait for a moment to keep the program running
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
}
