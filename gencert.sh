

openssl genrsa -out ca.key 2048
openssl req -new -x509 -key ca.key -out ca.crt -days 3560 -config ca.cnf

openssl genrsa -out server.key 2048
openssl req -new -key server.key -out server.csr -days 3650 -config server.cnf
openssl x509 -req -in server.csr -CA ca.crt -CAkey ca.key -CAcreateserial -out server.crt