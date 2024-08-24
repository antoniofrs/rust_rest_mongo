pub async fn hello_world() -> String {
    "Hello world".to_owned()
}

#[cfg(test)]
mod tests {
    use crate::routes::learn::hello_world::hello_world;

    #[tokio::test]
    async fn test_hello_world() {
        let result = hello_world().await;
        assert_eq!(result, "Hello world");
    }
}