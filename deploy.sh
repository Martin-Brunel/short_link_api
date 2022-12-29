git pull;
sudo systemctl stop rocket.service;
/home/ubuntu/.cargo/bin/diesel migration run;
cargo build --release;
sudo systemctl start rocket.service;
exit 0;