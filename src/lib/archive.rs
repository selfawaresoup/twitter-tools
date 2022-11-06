
use std::path::Path;
use std::fs;
use std::collections::HashMap;
use lazy_static::lazy_static;
pub mod types;
use types::*;

fn load_json_string(path: &str) ->  String {
	let json_path = Path::new(path);
	println!("Loading {}", path);
	let mut json = fs::read_to_string(json_path).expect("Loading failed");
	let cut_offset = json.find('[').expect("first [ not found");
	json.replace_range(..cut_offset, "");
	json
}

fn get_all_tweets() -> Vec<Tweet> {
	let json = load_json_string("data/archive/data/tweets.js");
	let entries:Vec<ArchiveEntry> = serde_json::from_str(&json).expect("JSON parsing failed");
	let mut tweets:Vec<Tweet> = vec![];
	
	for entry in entries {
		tweets.push(Tweet::from_archive_tweet(entry.tweet));
	}
	
	tweets
}

fn get_follower_ids() -> Vec<u64> {
	let json = load_json_string("data/archive/data/follower.js");
	let entries:Vec<FollowerEntry> = serde_json::from_str(&json).expect("JSON parsing failed");
	let mut ids:Vec<u64> = vec![];
	
	for entry in entries {
		ids.push(entry.follower.account_id.parse::<u64>().unwrap())
	}
	ids
}

fn get_following_ids() -> Vec<u64> {
	let json = load_json_string("data/archive/data/following.js");
	let entries:Vec<FollowingEntry> = serde_json::from_str(&json).expect("JSON parsing failed");
	let mut ids:Vec<u64> = vec![];
	
	for entry in entries {
		ids.push(entry.following.account_id.parse::<u64>().unwrap())
	}
	ids
}

fn find_threads(source_tweets: &Vec<Tweet>) -> Vec<Thread> {
	let mut threads: Vec<Thread> = vec![];
	let mut tweets = source_tweets.clone();
	tweets.sort_by(|b, a| a.created_at.cmp(&b.created_at)); //reverse sort by date

	while let Some(tweet) = tweets.pop() {
		if let Some(in_reply_to_status_id) = tweet.in_reply_to_status_id {
			for thread in threads.iter_mut() {
				if thread.has_id(in_reply_to_status_id) {
					thread.add(&tweet);
					break;
				}
			}
		} else {
			threads.push(Thread::new(&tweet));
		}
	}

	threads.into_iter().filter(|th| th.tweets.len() > 1).collect()
}

lazy_static! {
	static ref ARCHIVE: Archive = Archive::load();
}

#[derive(Clone)]
pub struct Archive {
	pub all_tweets: Vec<Tweet>,
	//tweets_by_id: HashMap<u64, Tweet>,
	follower_ids: Vec<u64>,
	pub following_ids: Vec<u64>,
	threads: Vec<Thread>
}

impl Archive {
	fn load() -> Archive {
		println!("Loading Archive …");
		let all_tweets = get_all_tweets();
		let mut tweets_by_id = HashMap::new();
		let follower_ids = get_follower_ids();
		let following_ids = get_following_ids();
		let threads = find_threads(&all_tweets);
		
		for tweet in &all_tweets {
			let id = tweet.id;
			tweets_by_id.insert(id, tweet.clone());
		}
		Archive{all_tweets, follower_ids, following_ids, threads}
	}
	
	#[allow(dead_code)]
	pub fn is_following(&self, id: u64) -> bool {
		self.following_ids.contains(&id)
	}

	pub fn find_thread_by_tweet(&self, tweet: &Tweet) -> Option<Thread> {
		for th in &self.threads {
			if th.has_id(tweet.id) {
				return Some(th.clone());
			}
		}
		None
	}
}

pub fn get_archive() -> Archive {
	ARCHIVE.clone()
}

pub struct Stats {}

impl Stats {
	pub fn run() {
		let archive = get_archive();
		println!("{} tweets in archive", archive.all_tweets.len());
		println!("{} followers", archive.follower_ids.len());
		println!("{} following", archive.following_ids.len());
		Self::thread_stats(&archive.threads);
	}

	fn thread_stats(threads: &Vec<Thread>) {
		let mut thread_stats:HashMap<u64, u64> = HashMap::new();

		for thread in threads {
			let l = thread.tweets.len() as u64;
			let count = match thread_stats.get(&l) {
				Some(c) => c + 1,
				None => 1
			};
			thread_stats.insert(l, count);
		}

		println!("{} threads total", threads.len());
		let mut stat_tuples:Vec<(&u64, &u64)> = thread_stats.iter().map(|(l, c)| (l, c)).collect();
		stat_tuples.sort_by(|b, a| a.0.cmp(b.0));

		for s in stat_tuples {
			println!("Length {}: {} threads", s.0, s.1);
		 }
	}
}
