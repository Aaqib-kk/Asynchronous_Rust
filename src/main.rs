// /* Example 1.1 */
// use std::thread;
// use std::time::Duration;

// fn get_two_sites() {
//     //spawn two threads to de work.
//     let thread_one = thread::spawn(|| {
//         thread::sleep(Duration::from_millis(500));
//         println!("Thread One")
//     });

//     let thread_two = thread::spawn(|| {
//         thread::sleep(Duration::from_millis(500));
//         println!("Thread Two")
//     });
//     let thread_three = thread::spawn(|| {
//         thread::sleep(Duration::from_millis(500));
//         println!("Thread Three")
//     });
//     let thread_four = thread::spawn(|| {
//         thread::sleep(Duration::from_millis(1000));
//         println!("Thread Four")
//     });

//     // wait for both threads to complete.println
//     thread_one.join().expect("Thread one panicked");
//     thread_two.join().expect("Thread two panicked");
//     thread_three.join().expect("Thread two panicked");
//     thread_four.join().expect("Thread two panicked");
// }

// fn main() {
//     get_two_sites();
// }

/* By using Asyn funtion */

// use std::time::Duration;
// use std::thread;
// use futures::executor::block_on;
// async fn download_async1(text : &str) {
//     thread::sleep(Duration::from_millis(2000));
//     println!("{:?}", text);
// }

// async fn download_async2(text : &str) {
//     println!("{:?}", text);
// }

// async fn download_async3(text : &str) {
//     println!("{:?}", text);
// }

// async fn get_two_sites_async() {
//     let future_one = download_async1("https://onesync.com");
//     let future_two = download_async2("https://twosync.com");
//     let future_three = download_async3("https://threesync.com");

//     futures::join!(future_one, future_two, future_three);
// }

// fn main () {
//     block_on(get_two_sites_async());
//     println!("Hello world!");
// }

// ====================================== .await ==================

// use futures::executor::block_on;
// use std::time::Duration;
// use std::thread;

// async fn hello_world() {
//     thread::sleep(Duration::from_millis(1000));
//     println!("hello world!");
// }

// async fn hello_IoT() {
//     println!("hello IoT!");
// }

// async fn another_function() {
//     hello_world().await;
//     hello_IoT().await;
// }

// fn main() {
//     block_on(another_function());
//     println!("Hello from Main")
// }

//=============================================================================
// use std::thread;
// use std::time::Duration;
// use futures::executor::block_on;

// async fn get_sum() -> f32 {
//     65.2
// }

// async fn calculate_grade(sum: f32) {
//     thread::sleep(Duration::from_millis(2000));
//     if sum > 50.0 {
//         println!("Candidate is passed");
//     }
//     else {
//         println!("Candidate is failed");
//     }
// }

// async fn printl_sum(sum: f32) {
//     println!("sum is {}", sum);
// }

// async fn get_sum_calcu_grade() {
//     let sum = get_sum().await;
//     calculate_grade(sum).await;
// }
// async fn oscillate()
// {
//     let f1 = get_sum_calcu_grade();
//     let f2 = printl_sum(52.2);
//     futures::join!(f2,f1);
// }
// fn main(){
//     block_on(oscillate());
// }

//=================learnm sing and dance========================
// use std::thread;
// use std::time::Duration;
// use futures::executor::block_on;

// async fn sing() -> String {
//     thread::sleep(Duration::from_secs(2));
//     "taray zameen pr".to_string()
// }
// async fn songgg(song:String) {
//     thread::sleep(Duration::from_secs(2));
//     println!("{}", song);
// }

// async fn dance() {
//     println!("dance");
// }

// async fn learn_and_sing() {
//     let song = sing().await;
//     songgg(song).await;

// }

// async fn aysnc_main() {
//     let f1 = learn_and_sing();
//     let f2 = dance();
//     futures::join!(f1,f2);
// }

// fn main()
// {
//     block_on(aysnc_main());
// }

//================= async rust====================

use std::thread;
use std::time::Duration;
use futures::executor::block_on;
use async_std::task;

// async fn negate_async(n: i32) -> i32 {
//     println!("Negating {}", n);
//     task::sleep(std::time::Duration::from_secs(5)).await;
//     println!("Finished Sleeping for {}!", n);
//     n* -1
// }

// async fn f() -> i32 {
//     let neg = negate_async(1);
//     let neg_task = task::spawn(negate_async(2));
//     task::sleep(std::time::Duration::from_secs(1)).await;
//     neg.await + neg_task.await
// }
// fn main() {
//     block_on(f());
// }

//===========================
async fn sing_song() {
    task::sleep(Duration::from_secs(0)).await;
    println!("Taray Zameen par");
}

async fn learn_song() {
    task::sleep(Duration::from_secs(0)).await;
    println!("Learning Song");
}

async fn learn_sing_song() {
    let f1 = learn_song().await;
    let f2 = sing_song().await;

}

async fn dance() {
    task::sleep(Duration::from_secs(2)).await;
    println!("Dancing");
}

async fn party() {
    let f1 = learn_sing_song();
    let f2 = dance();
    futures::join!(f1,f2);
}

fn main() {
    block_on(party());
}