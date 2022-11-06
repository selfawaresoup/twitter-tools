use lib::archive::get_archive;
use lib::auth::{auth_oauth1a};
use lib::actions::{DeleteTweet, DeleteRetweet, TweetNoOp, TweetAction};
use dotenv;
use chrono::offset::Utc;
use chrono::{Duration, DateTime, TimeZone};



pub struct DeleteOldTweets {

}

impl DeleteOldTweets {
  fn ancient_cutoff() -> DateTime<Utc> {
    let env_cutoff = dotenv::var("ANCIENT_CUTOFF").unwrap();
    println!("{}, {}", env_cutoff, env_cutoff.len());
    // let naive_cutoff = NaiveDateTime::parse_from_str(&env_cutoff, "%F").unwrap();
    // let naive_cutoff = NaiveDateTime::parse_from_str("2019-01-01", "%F").unwrap();
    // let ancient_cutoff = DateTime::from_utc(naive_cutoff, Utc);

    let ancient_cutoff = Utc.datetime_from_str(&env_cutoff, "%Y-%m-%d %H:%M:%S").unwrap();
    ancient_cutoff
  }

  pub fn run() {
    let archive = get_archive();
    let max_tweet_age = dotenv::var("MAX_TWEET_AGE").unwrap().parse::<u64>().unwrap();
    let minimum_likes = dotenv::var("MINIMUM_LIKES").unwrap().parse::<u64>().unwrap();
    let minimum_retweets = dotenv::var("MINIMUM_RETWEETS").unwrap().parse::<u64>().unwrap();
    let keep_tweets = dotenv::var("KEEP_TWEETS").unwrap();
    let now = Utc::now();

    let mut tweets_to_delete = vec![];
    let _no_op = Box::new(TweetNoOp{});
    let delete_tweet = Box::new(DeleteTweet{});
    let delete_retweet = Box::new(DeleteRetweet{});
    let ancient_cutoff = Self::ancient_cutoff();

    for tweet in &archive.all_tweets {
      let (likes, retweets) = match archive.find_thread_by_tweet(tweet) {
        Some(th) => (th.max_favorite_count(), th.max_retweet_count()),
        _ => (tweet.favorite_count, tweet.retweet_count)
      };

      if tweet.created_at < ancient_cutoff {continue;}
      if likes >= minimum_likes {continue;}
      if retweets >= minimum_retweets {continue;}
      if keep_tweets.contains(&tweet.id_str) {continue;}

      let tweet_created_utc: DateTime<Utc> = DateTime::from(tweet.created_at); 
      if now - tweet_created_utc <= Duration::days(max_tweet_age as i64) {continue};

      tweets_to_delete.push(tweet);
    }

    println!("{} tweets to delete", tweets_to_delete.len());
    
    for tweet in tweets_to_delete {
      match tweet.full_text.find("RT") {
        Some(index) => {
          if index == 0 {
            delete_retweet.run(auth_oauth1a(), &tweet);
            continue;
          }
        }
        _ => ()
      }
      delete_tweet.run(auth_oauth1a(), &tweet);
    }
  }
}