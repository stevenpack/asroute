cargo build --release
cd ../../target/release
tar -czf asroute-0.1.0-x86_64-apple-darwin.tar.gz asroute
cp asroute-0.1.0-x86_64-apple-darwin.tar.gz ../../pkg/brew
shasum -a 256 asroute-0.1.0-x86_64-apple-darwin.tar.gz