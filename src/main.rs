use openssl::ssl::{SslVerifyMode};
use tokio::{runtime::Handle};
use hyper::{self, Body, Request, body::HttpBody, client::Client};
use hyper::client::HttpConnector;
use anyhow::Error;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{Semaphore};
//use rand::Rng;
use waitgroup::WaitGroup;
use hyper_openssl::HttpsConnector;
use tracing_subscriber::FmtSubscriber;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PutRsp {
    #[serde(rename = "Len")]
    pub len: usize,
}

async fn send_req_https(c: Arc<Client<HttpsConnector<HttpConnector>>>, bufsz: usize, url: &str) -> Result<(), Error> {
    //let sz= rand::thread_rng().gen_range(10240..(2*1024*1024));
    let buf = vec![0u8; bufsz];

    let req = Request::builder()
        .method("POST")
        .uri(url)
        .body(hyper::Body::from(buf))?;

    
    let rsp = c.request(req).await?;
    let rsp_buf = rsp.into_body().data().await.unwrap()?;
    let _put_rsp: PutRsp = serde_json::from_slice(&rsp_buf)?;

    Ok(())
}

fn new_client() -> Result<Arc<Client<HttpsConnector<HttpConnector>>>, Error> {
    let mut https= hyper_openssl::HttpsConnector::new()?;
    https.set_callback(|conn, _| {
        conn.set_verify(SslVerifyMode::NONE);
        Ok(())
    });

    let client_builder = hyper::client::Client::builder();
    Ok(Arc::new(client_builder.build::<_, Body>(https)))
}

async fn test_https() -> Result<(), Error> {
    let url = std::env::var("HTEST_URL")?;
    let fut_limit: usize = std::env::var("HTEST_FUT_LIMIT")?.parse()?;
    let req_count: usize = std::env::var("HTEST_REQ_COUNT")?.parse()?;
    let bufsz: usize = std::env::var("HTEST_BUF_SIZE")?.parse()?;
    let conn_count: usize = std::env::var("HTEST_CONN_COUNT")?.parse()?;

    let mut client_vec = Vec::new();

    for _ in 0..conn_count {
        let c = new_client()?;
        client_vec.push(c)
    }



    let handle = Handle::current();
    let allowed = Arc::new(Semaphore::new(fut_limit));

    let wg = WaitGroup::new();

    for i in 0..conn_count {
        send_req_https(client_vec[i].clone(), bufsz, url.as_str()).await?;
    }

    for i in 0..req_count {
        if i % 100 == 0 {
            println!("i={}",i);
        }
        let permit = Semaphore::acquire_owned(allowed.clone()).await?;
        let worker = wg.worker();

        let cc = client_vec[i%conn_count].clone();

        //let cc = c.clone();
        let url_clone = url.clone();
        handle.spawn(async move {
            let _ = permit;
            let _ = worker;

            let res = send_req_https(cc, bufsz, url_clone.as_str()).await;
            if res.is_err() {
                println!("err = {:?}", res.err().unwrap());
            }
        });

    }

    wg.wait().await;

    //let total = bytes.lock().await;
    //println!("total bytes: {} in {:?}", *total, start.elapsed());
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(tracing::Level::WARN)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;
    

    let _ = test_https().await?;

    Ok(())
}