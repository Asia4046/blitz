cargo build --release 
cargo build --target x86_64-pc-windows-gnu --release
mv target/x86_64-pc-windows-gnu ./x86_64-pc-windows-gnu
cp -r target x86_64-pc-linux-gnu
tar -czvf blitz-release.tar.gz  x86_64-pc-windows-gnu x86_64-pc-linux-gnu
cargo clean 
rm -rf x86_64-pc-linux-gnu x86_64-pc-windows-gnu