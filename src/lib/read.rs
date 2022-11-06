use twitter_v2::TwitterApi;
use twitter_v2::authorization::{Oauth1aToken};
use twitter_v2::query::{TweetField, UserField};
use twitter_v2::{Tweet, User};
use twitter_v2::id::NumericId;
// use tokio::time::{sleep, Duration};
use async_std::task::block_on;

pub fn get_tweet(auth: Oauth1aToken, id: NumericId) -> Option<Tweet> {
	block_on(async{
		let api = TwitterApi::new(auth);
		let mut req = api.get_tweet(id);
		req.tweet_fields([TweetField::AuthorId, TweetField::CreatedAt]);	
		let response = req.send().await.unwrap();
		response.into_data()
	})
}

pub fn get_user(auth: Oauth1aToken, id: NumericId) -> Option<User> {
	block_on(async{
		let api = TwitterApi::new(auth);
		let mut req = api.get_user(id);
		req.user_fields([
			UserField::CreatedAt,
			UserField::Description,
			UserField::Username
		]);
		let response = req.send().await.unwrap();
		response.into_data()
	})
}

pub fn get_my_user(auth: Oauth1aToken) -> Option<User> {
	let id: u64 = dotenv::var("USER_ID").unwrap().parse().unwrap();
	get_user(auth, NumericId::new(id))
}

pub fn get_user_by_name(auth: Oauth1aToken, username: String) -> User {
	block_on(async {
		let api = TwitterApi::new(auth.to_owned());
		let mut req = api.get_user_by_username(username);
		req.user_fields([
			UserField::Username
		]);
		let res = req.send().await.unwrap();
		let user_data = res.into_data().unwrap();
		user_data
	})
}
