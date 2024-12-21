# Trading_Side
Trading_Side in Rust with Rabbitmq

Setup
Step 1: Install Rust
https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe

Step 2: Install Erlang
https://github.com/erlang/otp/releases/download/OTP-27.2/otp_win64_27.2.exe

Step 3: Install Rabbitmq
https://github.com/rabbitmq/rabbitmq-server/releases/download/v4.0.5/rabbitmq-server-4.0.5.exe

Step 4: Start Rabbitmq server
open RabbitMQ Command Prompt (sbin dir)
Example directory: C:\Program Files\RabbitMQ Server\rabbitmq_server-4.0.5\sbin
run command "rabbitmq-plugins.bat enable rabbitmq_management"
run command "rabbitmq-server start"
Go to browser, enter url "http://127.0.0.1:15672/" 
Login account with 
username:guest
password:guest

Step 5: Install vscode extension, rust-analyzer

Step 6: Install library dependencies with command "cargo build"

Step 7: Run the application with command "cargo run"
