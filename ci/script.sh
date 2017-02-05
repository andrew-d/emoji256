# This script takes care of testing your crate

set -ex

main() {
    cross build --target $TARGET
    cross build --target $TARGET --release

    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi

    cross test --target $TARGET
    cross test --target $TARGET --release

    tree target

    # We can't use 'cross run' because we need to pass a string to the binary
    # on stdin.  Just run the output manually.
    echo 'Foobar' | ./target/$TARGET/debug/emoji256
    echo 'Foobar' | ./target/$TARGET/release/emoji256
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
