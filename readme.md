# Twitter Tools

Requirements: Rust, Cargo

## Setup

1. copy `.env.example` to `.env` and fill in your Twitter API details, user id and user name
1. download your Twitter archive and copy/symlink it to `data/archive` in the project directory


## Usage

`cargo run <command>`

with `<command>` being one of the following:

### `archive-stats`

Extracts stats from your archive and prints them to the command line

### `block-with-followers <username>`

Blocks the specified user and all their followers, except those you already follow (based on your archive)

May take a long time due to API rate limits

### `block-liking-users <tweet_id>`

Blocks every user who has liked the specified tweet, except those you already follow (based on your archive)

### `delete-old-tweets`

Deletes tweets found in your archive according to the settings in `.env`:

Tweets younger than `MAX_TWEET_AGE` are skipped.

Tweets with at least `MINIMUM_LIKES` or `MINIMUM_RETWEETS` are skipped, as well as those that are part of a thread where at least one tweet satisfies those criteria.

Tweets listed in `KEEP_TWEETS` are skipped.

Tweets older that `ANCIENT_CUTOFF` are skipped. This setting is largely for cutting down on the time the script takes to run.

### `get-tweet <tweet_id>`

Prints details about the specified tweet (fetched from the API)

### `get-user <username>`

Prints details about the specified user (fetched from the API)

### `export-threads <min-length>`

Exports all threads from the archive that have at least `min-lengh` tweets as markdown files along with their attached images into the `export` directory.
 
