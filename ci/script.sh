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
    # on stdin, which doesn't currently work:
    #   https://github.com/japaric/cross/issues/52
    #
    # As a result, and since Linux can't run i686 binaries on a x64 host, we
    # blacklist the specific target.  It's possible we could add more logic
    # here, but this works for now.
    if [ "$TARGET" != "i686-unknown-linux-gnu" ]; then
        echo 'Foobar' | ./target/$TARGET/debug/emoji256
        echo 'Foobar' | ./target/$TARGET/release/emoji256
    fi
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
