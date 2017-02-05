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

    ls -alh ./target/$TARGET/debug || true
    ls -alh ./target/$TARGET/release || true

    # We can't use 'cross run' because we need to pass a string to the binary
    # on stdin.  Just run the output manually.
    if [ ! -x ./target/$TARGET/debug/emoji256 ]; then
        echo "Debug target doesn't exist!?"
    else
        echo 'Foobar' | ./target/$TARGET/debug/emoji256
    fi

    if [ ! -x ./target/$TARGET/release/emoji256 ]; then
        echo "Release target doesn't exist!?"
    else
        echo 'Foobar' | ./target/$TARGET/release/emoji256
    fi
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
