use futures::prelude::*;
use tokio::time::Duration;
// use tokio::prelude::*;
#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let res = tokio::spawn(async move {
        let ds1 = do_long_stuff().await;
        5 * ds1
    });
    println!("Doing other stufff");

    let res2 = res
        .map(|s| -> bool {
            println!("In mapping output");
            s.is_ok()
        })
        .map(|res| {
            print!("received {}", res);
            if res {
                print!("Evaluated success");
                1
            } else {
                print!("Evaluated success");
                -1
            }
        });

    let handle = tokio::spawn(async move { res2.await }).await;
    println!("Hanlde result is {:?}", handle);
    // let set = res2.await;
    //soft
    // print!("Result is {:?}", set); sadsa asdsa
    // let res3 = res.and_then(|r4| r4 * 2);

    ()
}
pub enum HelloWorld {}

async fn do_long_stuff() -> i32 {
    let _res = tokio::time::sleep(Duration::from_secs(2)).await;
    6
}
