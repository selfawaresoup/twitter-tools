use lib::archive::get_archive;

pub struct MostLiked {}

impl MostLiked {
	pub fn run(max: usize) {
		let archive = get_archive();
		let mut tweets = archive.all_tweets;
		tweets.sort_by(|b,a| a.favorite_count.cmp(&b.favorite_count));
		tweets.truncate(max);

		for tweet in tweets {
			println!("{}: {}", tweet.favorite_count, tweet.id);
			println!("{}\n", tweet.full_text);
		}
	}
}

pub struct MostRetweeted {}

impl MostRetweeted {
	pub fn run(max: usize) {
		let archive = get_archive();
		let mut tweets = archive.all_tweets;
		tweets.sort_by(|b,a| a.retweet_count.cmp(&b.retweet_count));
		tweets.truncate(max);

		for tweet in tweets {
			println!("{}: {}", tweet.retweet_count, tweet.id);
			println!("{}\n", tweet.full_text);
		}
	}
}