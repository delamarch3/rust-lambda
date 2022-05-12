for dir in $(ls -d */);
do
    cd $dir;
    cargo clean;
    cd ..;
done;
