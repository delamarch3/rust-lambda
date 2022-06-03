clean() {
    cd $dir;
    cargo clean;
    cd ..;
}

for dir in $(ls -d ../functions/*/);
do
    clean $dir &
done;
wait
