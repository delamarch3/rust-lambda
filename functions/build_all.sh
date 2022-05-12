for dir in $(ls -d */);
do
    cd $dir;
    cargo lambda build --release --target x86_64-unknown-linux-gnu --output-format zip;
    cd ..;
done;
