use twitter_v2::authorization::{Oauth1aToken, BearerToken};
use dotenv;

#[allow(dead_code)]
pub fn auth_bearer_token() -> BearerToken {
	let token = dotenv::var("BEARER_TOKEN").unwrap();
	BearerToken::new(token)
}

#[allow(dead_code)]
pub fn auth_oauth1a() -> Oauth1aToken {
	Oauth1aToken::new(
		dotenv::var("CONSUMER_KEY").unwrap(),
		dotenv::var("CONSUMER_SECRET").unwrap(),
		dotenv::var("ACCESS_TOKEN_KEY").unwrap(),
		dotenv::var("ACCESS_TOKEN_SECRET").unwrap(),
	)
}
