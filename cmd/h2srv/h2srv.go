package main

import (
	"encoding/json"
	"io/ioutil"
	"log"
	"net/http"
	"time"

	"github.com/gorilla/mux"
)

type PutRsp struct {
	Len int
}

//DataPut ...
func DataPut(w http.ResponseWriter, r *http.Request) {
	buf, err := ioutil.ReadAll(r.Body)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	defer r.Body.Close()

	rsp := &PutRsp{
		Len: len(buf),
	}

	time.Sleep(100 * time.Millisecond)
	json.NewEncoder(w).Encode(&rsp)
}

func startServer() {

	r := mux.NewRouter()

	r.HandleFunc("/put", DataPut).Methods("POST")

	log.Fatal(http.ListenAndServeTLS(":9001", "certs/server.crt", "certs/server.key", r))
	//log.Fatal(http.ListenAndServe(":9001", r))
}

func main() {
	startServer()
}
