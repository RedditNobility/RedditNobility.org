# RedditNobility

## Building

### Universal

1. Install Rust at https://www.rust-lang.org/
2. Follow OS specific steps first!
3. Setting up the Diesel
    1. Install Diesel: `cargo install diesel_cli`
4. Please add a .env file and use the below template for reference
5. Run `cargo build`

### Windows

1. Mysql C++ Library Install: https://dev.mysql.com/downloads/connector/cpp/
2. Set Mysql Dev Path inside the PATH variable.
3. Follow Steps here https://stackoverflow.com/a/61921362

### Example `.env`

```
DATABASE_URL={DATABSE URL- Read Diesel Docs} 
CLIENT_SECRET={Reddit Secret}
CLIENT_KEY={Reddit Client KEY}
REDDIT_USER={Reddit Username}
PASSWORD={REDDIT_PASSWORD}
URL={URL}
RECAPTCHA_SECRET={RECAOTCHA SECRET}
RECAPTCHA_PUB={RECAPTCHA PUB}
```
