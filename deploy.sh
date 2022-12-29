git pull;
sudo systemctl stop rocket.service;
diesel migration run;
cargo run --release;
sudo systemctl start rocket.service;
