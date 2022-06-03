build() {
    cd $1;
    cargo lambda build --release --target x86_64-unknown-linux-gnu --output-format zip;
    cd ..;
}

for dir in $(ls -d ../functions/*/);
do
    build $dir &
done;
wait
