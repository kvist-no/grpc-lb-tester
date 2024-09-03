# gRPC Load Balancing Tester

A very simple application that simply calls the `Info` service from [podinfo]
(https://github.com/stefanprodan/podinfo) over and over and logs the 
hostname. This is used to test that the gRPC load balancing works as expected.

You can set two environment variables to control this behaviour:

- `URL` which url to connect to
- `INTERVAL` how often to call the service in seconds