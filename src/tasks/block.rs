use lib::read::{get_user_by_name, get_my_user};
use lib::actions::{Block, UserNoOp, UserAction};
use lib::auth::{auth_oauth1a};
use lib::archive::get_archive;
use lib::related_users::{RelatedUsers, UserRelation};
use lib::delay::Delay;

pub struct BlockWithFollowers {
	
}

impl BlockWithFollowers {
	pub fn run(user_name: &String) {
		let target_user = get_user_by_name(auth_oauth1a(), user_name.to_owned());
		let block = Block{};
		let _no_op = UserNoOp{};
		let mut target_followers = RelatedUsers::new(auth_oauth1a(), target_user.to_owned(), UserRelation::Followers);

		let archive = get_archive();
		let my_following = archive.following_ids;
		let me = get_my_user(auth_oauth1a()).expect("My own user not found");
		let mut delay = Delay::new(18);

		block.run(auth_oauth1a(), &me, &target_user);

		while let Some(target_user) = target_followers.next() {
			delay.wait();
			if my_following.contains(&target_user.id.as_u64()) {
				println!("Skipping {}, since I'm folling them.", &target_user.name);
				continue;
			}

			block.run(auth_oauth1a(), &me, &target_user);
		}
	}
}