use futures::executor;
async fn async_add(left: i32, right: i32) -> i32 {
    left + right
}

async fn something_great_async_function() -> i32 {
    let ans = async_add(2, 3).await; // この時点で5という値を取り出せる。
                                     // 何か処理をはさむこともできる。
    println!("{}", ans);
    ans
}

fn main() {
    // 関数を実行する。5 が出力される。
    executor::block_on(something_great_async_function());
}
