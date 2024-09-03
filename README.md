# gRPC Load Balancing Tester

A very simple application that simply calls the `Info` and `Status` services from [podinfo](https://github.com/stefanprodan/podinfo) over and over and logs the
hostname and time to get hostname and time to get "Unavailable". This is used to test that the gRPC load balancing and
retries with Linkerd work as expected.

You can set two environment variables to control this behaviour:

- `URL` which url to connect to
- `INTERVAL` how often to call the service in seconds

This is meant to be run as a docker container in your kubernetes cluster, but you can use it locally as well. Either pull the latest docker image (see packages) or run the `grpc-lb-tester-client` binary with a Rust setup.
