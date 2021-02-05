package main

import (
	"bytes"
	"crypto/tls"
	"crypto/x509"
	"fmt"
	"io/ioutil"
	"log"
	"net/http"
	"sync"

	"golang.org/x/net/http2"
)

const url = "https://localhost:9001/put"

func doReq(c *http.Client) (err error) {
	raw := make([]byte, 256*1024)
	r := bytes.NewReader(raw)
	req, err := http.NewRequest("POST", url, r)
	if err != nil {
		fmt.Println(err)
		return
	}

	// Perform the request
	resp, err := c.Do(req)
	if err != nil {
		fmt.Println(err)
		return
	}
	defer resp.Body.Close()
	_, err = ioutil.ReadAll(resp.Body)
	if err != nil {
		log.Fatalf("Failed reading response body: %s", err)
	}
	return
}

func main() {
	client := &http.Client{}

	// Create a pool with the server certificate since it is not signed
	// by a known CA
	caCert, err := ioutil.ReadFile("certs/ca.crt")
	if err != nil {
		log.Fatalf("Reading server certificate: %s", err)
	}
	caCertPool := x509.NewCertPool()
	caCertPool.AppendCertsFromPEM(caCert)

	// Create TLS configuration with the certificate of the server
	tlsConfig := &tls.Config{
		RootCAs: caCertPool,
	}

	// Use the proper transport in the client
	client.Transport = &http2.Transport{
		TLSClientConfig:            tlsConfig,
		StrictMaxConcurrentStreams: true,
	}

	count := 600
	allow := make(chan bool, count)
	for i := 0; i < count; i++ {
		allow <- true
	}

	wg := &sync.WaitGroup{}

	for i := 0; i < 100000; i++ {
		wg.Add(1)
		_ = <-allow
		if i%200 == 0 {
			fmt.Println(i)
		}

		go func() {
			doReq(client)
			allow <- true
			wg.Done()
		}()
	}

	wg.Wait()
}
