use async_trait::async_trait;
#[async_trait]
trait AsyncTrait {
    async fn f() {
        println!("Could compile");
    }
}
