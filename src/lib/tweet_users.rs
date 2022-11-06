use twitter_v2::TwitterApi;
use twitter_v2::authorization::{Oauth1aToken};
use twitter_v2::{User, Tweet};
use twitter_v2::query::{UserField};
use crate::delay::Delay;
use async_std::task::block_on;

pub enum TweetRelation {
	Like,
	Retweet
}

pub enum UserAction {
	Block,
	Print
}

pub struct TweetUsers {
	pub relation: TweetRelation,
	pub tweet: Tweet,
	pagination_token: Option<String>,
	ended: bool,
	users: Vec<User>,
	api: Option<TwitterApi<Oauth1aToken>>,
	delay: Delay,
}

impl TweetUsers {
	pub fn new(tweet: Tweet, relation: TweetRelation) -> TweetUsers {
		TweetUsers{
			tweet,
			relation,
			pagination_token: None,
			ended: false,
			users: vec![],
			api: None,
			delay: Delay::new(60)
		}
	}
	
	pub fn reset(&mut self) {
		self.users =  vec![];
		self.api = None;
		self.ended = false;
		self.pagination_token = None;
	}
	
	fn fetch(&mut self) {
		block_on(async {
			self.delay.wait();
			let api = self.api.as_ref().unwrap();
			let mut req = match self.relation {
				TweetRelation::Like => api.get_tweet_liking_users(self.tweet.id),
				TweetRelation::Retweet => api.get_tweet_retweeted_by(self.tweet.id)
			};
			req.user_fields([
				UserField::Username
			]);
			
			// println!("Pagination token: {}", self.pagination_token.to_owned().unwrap_or(String::from("<None>")));
			
			if let Some(t) = self.pagination_token.to_owned() {
				req.pagination_token(&t);
				println!("Request paginated, waiting 60 secs …");
			}
			
			let res = req.send().await.unwrap();
			let meta_data = res.meta().unwrap();
			self.pagination_token = meta_data.next_token.to_owned();
			let related_users = res.data().unwrap();
			
			println!("Fetched {} related_users", related_users.len());
			
			self.users.append(&mut related_users.to_owned());
			
			match self.pagination_token {
				Some(_) => {},
				None => {
					self.ended = true;
				}
			}
		})
	}
}

// impl Iterator for TweetUsers {
// 	type Item = User;

// 	fn next(&mut self) -> Option<Self::Item> {
// 		if self.ended { return None; }
	
// 		if self.batch.len() == 0 {
// 			self.fetch();
// 		}

// 		self.batch.pop_front()
// 	}
// }
