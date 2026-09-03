# Log sources

The transcripts in this directory are real build output, recorded by actually
running the tools. Nothing here is hand-written.

## autotools.log / error/autotools.log

GNU coreutils 9.5 (https://ftp.gnu.org/gnu/coreutils/coreutils-9.5.tar.gz),
recorded on macOS/arm64 with Apple clang:

- `autotools.log` - `./configure --disable-nls` followed by `make`
- `error/autotools.log` - `./configure --disable-nls --with-openssl=yes`,
  which stops at `checking for MD5 in -lcrypto... no`

## cmake.log / error/cmake.log

curl 8.10.1 (https://curl.se/download/curl-8.10.1.tar.gz), configured with
`-DCURL_ENABLE_SSL=OFF`:

- `cmake.log` - `cmake` configure followed by `cmake --build .`
- `error/cmake.log` - the same configure, then a build where `lib/escape.c`
  was edited to call an undeclared function, so clang and make report the
  real failure cascade