
# Repo to reproduce the stalling of Hyper Client

## Layout of the repo

* `./cmd/h2srv` => Go Http Server

* `./cmd/h2go` => Go Http Client

* `./src/main.rs` => Rust Http Client

## What happens in this client

A simple REST API which expects byte buffer in a body and responds back length of the buffer in a json.

Example: Send "hello" and get its length back

```bash
$ echo hello | curl -X POST -k --http2 --data-binary @- $HTEST_URL
{"Len":6}
```


## Building 

1. Generate certs first (openssl needed)

```bash
cd certs
bash ../gencert.sh
```

2. Build server

```bash
go build ./cmd/dtsrv
```

3. Build go client

```bash
go build ./cmd/h2go
```

4. Build rust client

```bash
cargo build --release
```

## Running

1. Running server

```bash
./dtsrv
```

2. Running Rust Client

```bash
source test.env
./target/release/hyper-stuck
```

3. Running Go client

```bash
export GODEBUG=x509ignoreCN=0
./h2go
```

## Params for the hyper client

The are in the file `test.env`. They are:

```
# The URL to connect to
export HTEST_URL=https://127.0.0.1:9001/put

# The number outstanding requests. 1 future => 1 request
export HTEST_FUT_LIMIT=400

# Number of requests to be made
export HTEST_REQ_COUNT=100000

# Size of the buffer sent in each request (below is 256KB)
export HTEST_BUF_SIZE=$((256*1024))

# Number of connections. In hyper, this will create as many hyper::client::Client instances
export HTEST_CONN_COUNT=1
```
