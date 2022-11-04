use twitter_v2::{Tweet, User};

pub fn print_tweet(tweet: Tweet) {
  println!("Created at: {}", tweet.created_at.unwrap());
  println!("Author ID: {}", tweet.author_id.unwrap());
	println!("{}", tweet.text);
}

pub fn print_user(user: User) {
  println!("ID: {}", user.id);
  println!("Name: {}", user.name);
  println!("Name: @{}", user.username);
  match user.created_at {
    Some(c) => { println!("Created at: {}", c); },
    _ => ()
  }
  match user.description {
    Some(d) => { println!("Name: @{}", d) },
    _ => ()
  }
}
