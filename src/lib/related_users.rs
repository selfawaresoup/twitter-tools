use twitter_v2::{TwitterApi};
use twitter_v2::authorization::{Oauth1aToken};
use twitter_v2::User;
use twitter_v2::query::{UserField};
use async_std::task::block_on;
use std::collections::VecDeque;
use crate::delay::Delay;

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
	batch: VecDeque<User>,
	api: Option<TwitterApi<Oauth1aToken>>,
	delay: Delay,
}

impl RelatedUsers {
	pub fn new(auth: Oauth1aToken, user: User, relation:UserRelation) -> RelatedUsers {
		let api = Some(TwitterApi::new(auth.clone()));
		RelatedUsers{
			api,
			user,
			relation,
			pagination_token: None,
			ended: false,
			batch: VecDeque::new(),
			delay: Delay::new(60),
		}
	}
	
	fn fetch(&mut self) {
		block_on(async {
			self.delay.wait();
			let api = self.api.as_ref().unwrap();
			let mut req = match self.relation {
				UserRelation::Followers => api.get_user_followers(self.user.id),
				UserRelation::Following => api.get_user_following(self.user.id)
			};
			req.user_fields([
				UserField::Username
			]);
			
			if let Some(t) = self.pagination_token.to_owned() {
				req.pagination_token(&t);
			}
			
			let res = req.send().await.unwrap();
			let meta_data = res.meta().unwrap();
			self.pagination_token = meta_data.next_token.to_owned();
			let related_users = res.data().unwrap();
			
			println!("Fetched {} related_users", related_users.len());
			for user in related_users {
				self.batch.push_back(user.to_owned());
			}
			
			match self.pagination_token {
				Some(_) => {},
				None => {
					self.ended = true;
				}
			}
		})
	}
}

impl Iterator for RelatedUsers {
	type Item = User;

	fn next(&mut self) -> Option<Self::Item> {
		if self.batch.len() == 0 {
			if self.ended { return None; }
			self.fetch();
		}

		self.batch.pop_front()
	}
}
