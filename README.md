# RedditNobility

## Building
---
# Requirements to Build or To Contribute
1. Rust 1.56 or newer installed
2. Mysql C++ Driver/Connector installed
3. Mysql Database Ready for use
4. For SSL openssl library installed
5. Node 16 installed and NPM  installed
6. Lots of Patience.
# Setup for Build
1. Pull the latest code and go into the site directory. Execute `npm install`
2. If on Linux execute the build.sh add the argument `ssl` if you want ssl support
3. if on Windows. Execute `npm run build` in the site directory. Then `cargo build --release` for the final build. Add --features ssl if you want ssl
4. After the build is complete an executable will be available at `target/release/rn_site` This is your website
# Configuring Website
1. Copy example.env to your working directory of the application and name it .env
2. The only one you will need to edit will be the `DATABASE_URL` and BIND_URL if that port is already in use



### Example `.env`

```
DATABASE_URL={DATABSE URL- Read Diesel Docs} 
# Location of the Frontend
SITE_DIR=../site/dist
LOG_LOCATION="./"
# Binding Address
ADDRESS="0.0.0.0:6742"

# Reddit Login Details
CLIENT_SECRET={Reddit Secret}
CLIENT_KEY={Reddit Client KEY}
REDDIT_USER={Reddit Username}
PASSWORD={REDDIT_PASSWORD}

# System Mode
MODE=DEBUG
```
