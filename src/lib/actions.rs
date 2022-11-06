use twitter_v2::{User};
use twitter_v2::TwitterApi;
use twitter_v2::authorization::{Oauth1aToken};
use crate::archive::types::Tweet as ArchiveTweet;
use dotenv;
use async_std::task::block_on;

pub trait UserAction {
  fn run(&self, auth: Oauth1aToken, source_user: &User, target_user: &User) -> bool;
}

pub struct Block {}

impl UserAction for Block {
  fn run(&self, auth: Oauth1aToken, source_user: &User, target_user: &User) -> bool {
    block_on(async{
      let api = TwitterApi::new(auth);
      println!("Blocking user {}", target_user.id);
      let res = api.post_user_blocking(source_user.id, target_user.id).await;
      
      match res {
        Ok(result) => {
          let blocking = result.data().unwrap();
          println!("{}", blocking.as_bool());
        },
        Err(err) => {println!("ERROR: {}", err)}
      }
      true
    })
  }
}

pub struct UserNoOp {}

impl UserNoOp {}

impl UserAction for UserNoOp {
  fn run(&self, _auth: Oauth1aToken, _source_user: &User, target_user: &User) -> bool {
    println!("NoOp on user {}", &target_user.name);
    true
  }
}

pub trait TweetAction {
  fn run(&self, auth: Oauth1aToken, tweet: &ArchiveTweet) -> bool;
}

pub struct DeleteTweet {}

impl TweetAction for DeleteTweet {
  fn run(&self, auth: Oauth1aToken, tweet: &ArchiveTweet) -> bool {
    block_on(async {
      let api = TwitterApi::new(auth);
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
      true
    })
  }
}

pub struct DeleteRetweet {}

impl TweetAction for DeleteRetweet {
  fn run(&self, auth: Oauth1aToken, tweet: &ArchiveTweet) -> bool {
    block_on(async {
      let api = TwitterApi::new(auth);
      let user_id = dotenv::var("USER_ID").unwrap().parse::<u64>().unwrap();
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
      true
    })
  }
}

pub struct TweetNoOp {}

impl TweetAction for TweetNoOp {
  fn run(&self, _auth: Oauth1aToken, tweet: &ArchiveTweet) -> bool {
    block_on(async {
      println!("NoOp on tweet {}, {}", tweet.id, tweet.created_at);
      true
    })
  }
}