use serde::Deserialize;
use chrono::{DateTime, FixedOffset};

#[derive(Debug, Deserialize)]
pub struct ArchiveEntry {
  pub tweet: ArchiveTweet
}

#[derive(Debug, Deserialize)]
pub struct ArchiveTweet {
  id_str: String,
  full_text: String,
  in_reply_to_status_id_str: Option<String>,
  created_at: String,
  favorite_count: String,
  retweet_count: String
}

#[derive(Clone)]
pub struct Tweet {
  pub id: u64,
  pub id_str: String,
  pub full_text: String,
  pub in_reply_to_status_id: Option<u64>,
  pub in_reply_to_status_id_str: Option<String>,
  pub created_at: DateTime::<FixedOffset>,
  pub favorite_count: u64,
  pub retweet_count: u64,
}

impl Tweet {
  pub fn from_archive_tweet(at: ArchiveTweet) -> Tweet {
    let id = at.id_str.parse::<u64>().unwrap();
    let id_str = at.id_str.clone();
    let full_text = at.full_text.clone();
    let in_reply_to_status_id_str = at.in_reply_to_status_id_str.clone();
    let in_reply_to_status_id: Option<u64> = match at.in_reply_to_status_id_str {
      Some(s) => Some(s.parse::<u64>().expect("parsing id failed")),
      None => None,
    };
    let created_at = DateTime::parse_from_str(&at.created_at, "%a %b %d %T %z %Y").expect("Date parsing failed");

    let favorite_count = at.favorite_count.parse::<u64>().unwrap();
    let retweet_count = at.retweet_count.parse::<u64>().unwrap();

    Tweet{
      id,
      id_str,
      full_text,
      in_reply_to_status_id,
      in_reply_to_status_id_str,
      created_at,
      favorite_count,
      retweet_count,
    }
  }
}

#[derive(Clone)]
pub struct Thread {
  pub tweets: Vec<Tweet>,
}

impl Thread {
  pub fn new(tweet: &Tweet) -> Thread {
    let tweets = vec![tweet.clone()];
    Thread{tweets}
  }

  pub fn add(&mut self, tweet: &Tweet) {
    self.tweets.push(tweet.clone());
  }

  pub fn has_id(&self, id:u64) -> bool {
    for tweet in &self.tweets {
      if tweet.id == id {return true};
    }

    false
  }

  pub fn max_favorite_count(&self) -> u64 {
    let mut count = 0;
    for tweet in &self.tweets {
      if tweet.favorite_count > count {
        count = tweet.favorite_count;
      }
    }

    count
  }
  
  pub fn max_retweet_count(&self) -> u64 {
    let mut count = 0;
    for tweet in &self.tweets {
      if tweet.retweet_count > count {
        count = tweet.retweet_count;
      }
    }

    count
  }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountIdEntry {
  pub account_id: String,
}

#[derive(Debug, Deserialize)]
pub struct FollowerEntry {
  pub follower: AccountIdEntry
}

#[derive(Debug, Deserialize)]
pub struct FollowingEntry {
  pub following: AccountIdEntry
}