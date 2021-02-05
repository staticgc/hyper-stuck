
# Repo to reproduce the stalling of Hyper Client

## Layout of the repo

./cmd/h2srv => Go Http Server
./cmd/h2go => Go Http Client
./src/main.rs => Rust Http Client

## What happens in this client

A simple REST API which expects byte buffer in a body and responds back length of the buffer in a json.

Example: Send "hello" and get its length back

```bash
$ echo hello | curl -X POST -k --http2 --data-binary @- $HTEST_URL
{"Len":6}
```


## Building and running

1. Generate Certs first


## Params for the hyper client

The are in the file `test.env`. They are:

```
# The URL to connect to
export HTEST_URL=https://127.0.0.1:9001/put

# The number outstanding requests. 1 future => 1 request
export HTEST_FUT_LIMIT=400

# Number of requests to be made
export HTEST_REQ_COUNT=100000

# 
export HTEST_BUF_SIZE=$((256*1024))
export HTEST_CONN_COUNT=1
```
