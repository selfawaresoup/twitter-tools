use twitter_v2::TwitterApi;
use twitter_v2::authorization::{Oauth1aToken};
use twitter_v2::User;
use twitter_v2::query::{UserField};
use tokio::time::{sleep_until, Duration, Instant};
use lib::actions::{UserAction};
use lib::read::{get_my_user};

#[allow(dead_code)]
pub enum UserRelation {
	Followers,
	Following,
}

pub struct RelatedUsers {
	pub relation: UserRelation,
	pub user: User,
	pagination_token: Option<String>,
	ended: bool,
	users: Vec<User>,
	batch: Vec<User>,
	api: Option<TwitterApi<Oauth1aToken>>,
	last_request_time: Option<Instant>,
}

impl RelatedUsers {
	pub fn new(user: User, relation:UserRelation) -> RelatedUsers {
		RelatedUsers{
			user,
			relation,
			pagination_token: None,
			ended: false,
			users: vec![],
			batch: vec![],
			api: None,
			last_request_time: None
		}
	}
	
	#[allow(dead_code)]
	pub fn reset(&mut self) {
		self.users =  vec![];
		self.batch =  vec![];
		self.api = None;
		self.ended = false;
		self.pagination_token = None;
	}
	
	async fn fetch(&mut self) {
		let api = self.api.as_ref().unwrap();
		let mut req = match self.relation {
			UserRelation::Followers => api.get_user_followers(self.user.id),
			UserRelation::Following => api.get_user_following(self.user.id)
		};
		req.user_fields([
			UserField::Username
		]);
		
		// println!("Pagination token: {}", self.pagination_token.to_owned().unwrap_or(String::from("<None>")));

		if let Some(t) = self.last_request_time {
			let next_request_time = t + Duration::from_secs(60);
			let remaining_sleep = next_request_time - Instant::now();
			if remaining_sleep.as_millis() > 0 {
				println!("Throttling, waiting {}s …", remaining_sleep.as_secs());
				sleep_until(next_request_time).await; // pause if this is a paginated request to prevent API throttling
			}
		}
		
		if let Some(t) = self.pagination_token.to_owned() {
			req.pagination_token(&t);
		}

		let res = req.send().await.unwrap();
		let meta_data = res.meta().unwrap();
		self.pagination_token = meta_data.next_token.to_owned();
		let related_users = res.data().unwrap();
		
		println!("Fetched {} related_users", related_users.len());

		self.users.append(&mut related_users.to_owned());
		self.batch.append(&mut related_users.to_owned());

		match self.pagination_token {
			Some(_) => {},
			None => {
				self.ended = true;
			}
		}

		self.last_request_time = Some(Instant::now());
	}

	#[allow(dead_code)]
	pub async fn each(&mut self, auth: Oauth1aToken, action: Box<dyn UserAction>) -> () {
		let ignore_ids: Vec<u64> = vec![];
		self.each_with_ignore_list(auth, action, ignore_ids).await
	}
		
	pub async fn each_with_ignore_list(&mut self, auth: Oauth1aToken, action: Box<dyn UserAction>, ignore_ids: Vec<u64>) -> () {
		self.api = Some(TwitterApi::new(auth.clone()));
		let me = get_my_user(auth.clone()).await.unwrap();
		
		loop {
			if self.ended && self.batch.is_empty() { break; }

			if self.batch.is_empty() {
				println!("Fetching more users …");
				self.fetch().await;
			}

			for u in &self.batch {
				if ignore_ids.contains(&u.id.as_u64()) {
					println!("Skipping {}, since I'm folling them.", &u.name);	
					continue;
				}
				action.run(auth.to_owned(), &me, u).await;
			}

			self.batch.clear();
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
