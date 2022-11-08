use lib::archive::get_archive;
use std::fs::{create_dir_all, write, copy};

pub struct ExportThreads {}

impl ExportThreads {
	pub fn run(min_length: u64) {
		create_dir_all("export").expect("Creating export dorectory failed");

		let archive = get_archive();
		
		for thread in archive.threads {
			if (thread.tweets.len() as u64) < min_length {continue;}
			
			let first = thread.tweets.first().unwrap().to_owned();
			let len = thread.tweets.len();
			let mut thread_md = String::new();
			for tweet in thread.tweets {
				for m in tweet.media {
					let source_file_name = format!("data/archive/data/tweets_media/{}", m.file_name);
					let target_file_name = format!("export/{}", m.file_name);
					println!("{}", source_file_name);
					copy(source_file_name, target_file_name).unwrap_or_default();
					let embed = format!("![]({})", m.file_name);
					thread_md.push_str(&embed);
					thread_md.push_str("\n");
				}
				thread_md.push_str(&tweet.full_text);
				thread_md.push_str("\n\n");
			}
			
			let filename = format!("export/{}_{}_{}.md", first.id_str, first.created_at.format("%Y-%m-%d"), len);
			write(filename, thread_md).expect("Writing export file failed");
		}
	}
}
