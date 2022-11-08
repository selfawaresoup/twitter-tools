mod args;
mod tasks;
use args::{read_args, Command};
use lib::auth::{auth_oauth1a};
use lib::read::{get_tweet, get_user_by_name};
use lib::print::{print_tweet, print_user};
use twitter_v2::id::NumericId;
use tasks::delete::DeleteOldTweets;
use tasks::block::{BlockWithFollowers, BlockLikers};
use lib::archive::{Stats};

const HELP: &'static str = include_str!("help.txt");

fn main() {
	let args = read_args();
	match args.command {
		Command::Unknown => {
			println!("Unknown command");
			std::process::exit(1);
		}
		
		Command::Help => println!("{HELP}"),
		
		Command::GetTweet => {
			let id = args.options[0].parse::<u64>().expect("ID needs to be a positive integer number");
			let tweet = get_tweet(auth_oauth1a(), NumericId::new(id)).unwrap();
			print_tweet(tweet);
		},
		
		Command::GetUser => {
			let user_name: String = args.options[0].to_owned();
			let user = get_user_by_name(auth_oauth1a(), user_name);
			print_user(user);
		},
		
		Command::BlockWithFollowers => {
			let user_name: String = args.options[0].to_owned();
			BlockWithFollowers::run(&user_name);
		},

		Command::BlockLikingUsers => {
			let tweet_id: u64 = args.options[0].to_owned().parse::<u64>().expect("Couldn't parse ID");
			BlockLikers::run(tweet_id);
		},
		
		Command::DeleteOldTweets => {
			DeleteOldTweets::run();
		},
		
		Command::ArchiveStats => {
			Stats::run();
		},
		_ => {
			println!("Not implemented yet");
		}
	};
}
