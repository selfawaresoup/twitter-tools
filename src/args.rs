pub enum Command {
  Unknown,
	Help,
	GetTweet,
  GetUser,
  BlockWithFollowers,
  GetAllFollowers,
  BlockLikingUsers,
  DeleteOldTweets,
  ArchiveStats,
  ExportThreads,
}

pub struct CliArgs {
	pub command: Command,
	pub options: Vec<String>
}

pub fn read_args() -> CliArgs {
	use std::env::args;
	let args: Vec<String> = args().collect();
	
	let command = match args.len() {
		1 => Command::Help,
    2 => match args[1].as_str() {
      "help" => Command::Help,
      "archive-stats" => Command::ArchiveStats,
      "delete-old-tweets" => Command::DeleteOldTweets,
      _ => Command::Unknown
    },
		3 => match args[1].as_str() {
      "get-tweet" => Command::GetTweet,
      "get-user" => Command::GetUser,
      "block-with-followers" => Command::BlockWithFollowers,
      "block-liking-users" => Command::BlockLikingUsers,
      "get-all-followers" => Command::GetAllFollowers,
      "export-threads" => Command::ExportThreads,
			_ => Command::Unknown
		}
		_ => Command::Unknown
	};

  let options = if args.len() > 2 {
    let mut args2: Vec<String> = args;
    args2.remove(0);
    args2.remove(0);
    //chopped off args 0 an 1
    args2
  } else {
    vec![]
  };

	CliArgs{command, options}
}
