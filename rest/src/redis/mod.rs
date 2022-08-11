extern crate redis;
static mut db: std::result::Result<redis::Client, redis::RedisError> = redis::Client::open("redis://127.0.0.1:6379/");

pub fn main() -> redis::RedisResult<()> {
    let con = db.unwrap().get_connection()?;

    Ok(())
}