use twitter_v2::{User};
use twitter_v2::TwitterApi;
use twitter_v2::authorization::{Oauth1aToken};
use async_trait::async_trait;
use tokio::time::{sleep_until, Duration, Instant};
use crate::archive::types::Tweet as ArchiveTweet;
use dotenv;

#[async_trait]
pub trait UserAction {
  async fn run(&self, auth: Oauth1aToken, source_user: &User, target_user: &User) -> bool;
}

pub struct Block {}

#[async_trait]
impl UserAction for Block {
  async fn run(&self, auth: Oauth1aToken, source_user: &User, target_user: &User) -> bool {
    let api = TwitterApi::new(auth);
    let start = Instant::now();
    println!("Blocking user {}", target_user.id);
    let res = api.post_user_blocking(source_user.id, target_user.id).await;

    match res {
      Ok(result) => {
        let blocking = result.data().unwrap();
        println!("{}", blocking.as_bool());
      },
      Err(err) => {println!("ERROR: {}", err)}
    }
    //API limit 50 reqs per 15min: 1 per 18 secs
    sleep_until(start + Duration::from_millis(18000)).await;
    true
  }
}

pub struct UserNoOp {}

#[async_trait]
impl UserAction for UserNoOp {
  async fn run(&self, _auth: Oauth1aToken, _source_user: &User, target_user: &User) -> bool {
    println!("NoOp on user {}", &target_user.name);
    true
  }
}

#[async_trait]
pub trait TweetAction {
  async fn run(&self, auth: Oauth1aToken, tweet: &ArchiveTweet) -> bool;
}

pub struct DeleteTweet {}

#[async_trait]
impl TweetAction for DeleteTweet {
  async fn run(&self, auth: Oauth1aToken, tweet: &ArchiveTweet) -> bool {
    let api = TwitterApi::new(auth);
    let start = Instant::now();
    println!("Deleting tweet {}, {}", tweet.id, tweet.created_at);
    println!("{}", tweet.full_text);
    let res = api.delete_tweet(tweet.id).await;

    match res {
      Ok(result) => {
        let deleted = result.data().unwrap();
        println!("Deleted: {}", deleted.as_bool());
      },
      Err(err) => {println!("ERROR: {}", err)}
    }
    //API limit 50 reqs per 15min: 1 per 18 secs
    sleep_until(start + Duration::from_millis(18000)).await;
    true
  }
}

pub struct DeleteRetweet {}

#[async_trait]
impl TweetAction for DeleteRetweet {
  async fn run(&self, auth: Oauth1aToken, tweet: &ArchiveTweet) -> bool {
    let user_id = dotenv::var("USER_ID").unwrap().parse::<u64>().unwrap();
    let api = TwitterApi::new(auth);
    let start = Instant::now();
    println!("Deleting retweet {}, {}", tweet.id, tweet.created_at);
    println!("{}", tweet.full_text);
    let res = api.delete_user_retweet(user_id, tweet.id).await;

    match res {
      Ok(result) => {
        let deleted = result.data().unwrap();
        println!("Retweeted: {}", deleted.as_bool());
      },
      Err(err) => {println!("ERROR: {}", err)}
    }
    //API limit 50 reqs per 15min: 1 per 18 secs
    sleep_until(start + Duration::from_millis(18000)).await;
    true
  }
}

pub struct TweetNoOp {}

#[async_trait]
impl TweetAction for TweetNoOp {
  async fn run(&self, _auth: Oauth1aToken, tweet: &ArchiveTweet) -> bool {
    println!("NoOp on tweet {}, {}", tweet.id, tweet.created_at);
    true
  }
}