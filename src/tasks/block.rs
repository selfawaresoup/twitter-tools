use lib::read::{get_user_by_name};
use lib::actions::{Block, UserNoOp};
use lib::auth::{auth_oauth1a};
use lib::archive::get_archive;
use crate::related_users::{RelatedUsers, UserRelation};

pub struct BlockWithFollowers {
	
}

impl BlockWithFollowers {
	pub async fn run(user_name: &String) {
		let target_user = get_user_by_name(auth_oauth1a(), user_name.to_owned()).await;
		let _block = Box::new(Block{});
		let no_op = Box::new(UserNoOp{});
		let mut target_followers = RelatedUsers::new(target_user, UserRelation::Followers);

		let archive = get_archive();
		let my_following = archive.following_ids;

		target_followers.each_with_ignore_list(auth_oauth1a(), no_op, my_following).await;
	}
}