use twitter_v2::TwitterApi;
use twitter_v2::authorization::{Oauth1aToken};
use twitter_v2::{User, Tweet};
use twitter_v2::query::{UserField};
use tokio::time::{sleep, Duration};

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
		}
	}
	
	pub fn reset(&mut self) {
		self.users =  vec![];
		self.api = None;
		self.ended = false;
		self.pagination_token = None;
	}
	
	async fn fetch(&mut self) {
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
			sleep(Duration::from_secs(60)).await; // pause if this is a paginated request to prevent API throttling
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
	}
		
	pub async fn each(&mut self, auth: Oauth1aToken, action: UserAction) -> ()
	{
		self.api = Some(TwitterApi::new(auth));
		
		loop {
			if self.ended && self.users.is_empty() { break; }

			if self.users.is_empty() {
				println!("fetching");
				self.fetch().await;
			} else {
				self.action(&action).await;
			}
		}
	}

	async fn action(&mut self, action: &UserAction) -> () {
		let u = self.users.remove(0);
		match action {
			UserAction::Print => {
				println!("{}", u.username);
			},
			UserAction::Block => {
				println!("Blocking {} ...", u.username);
				sleep(Duration::from_secs(1)).await;
			}
		}

	}

	pub async fn get_all(&mut self, auth: Oauth1aToken) -> Vec<User> {
		self.api = Some(TwitterApi::new(auth));
		
		loop {
			if self.ended { break }
			self.fetch().await;
		}

		self.users.to_owned()
	}
}
