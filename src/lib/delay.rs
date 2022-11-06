use chrono::{Duration, DateTime};
use chrono::offset::Utc;

pub struct Delay {
	delay_time: Duration,
	last_call: Option<DateTime<Utc>>
}

impl Delay {
	pub fn new(delay_seconds: u64) -> Delay {
		let delay_time = Duration::seconds(delay_seconds as i64);
		Delay{delay_time, last_call: None}
	}

	pub fn wait(&mut self) {
		if let Some(last) = self.last_call {
			let waited = Utc::now() - last;
			if waited < self.delay_time {
				let to_wait = self.delay_time - waited;
				println!("Waiting for {} seconds", to_wait.num_seconds());
				std::thread::sleep(to_wait.to_std().unwrap())
			}
		}

		self.last_call = Some(Utc::now());
	}
}
