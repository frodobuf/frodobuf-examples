This is an example of a capability provider using frodobuf

The calculator provider interface is defined in
calc-interface/calc.midl.

Because actors are event-driven, we need something to trigger the actor,
so we have it implement HttpServer and send it an HttpRequest.

# Running

To build and run, you will need both `wash` and `wasmcloud` in your
path. 
You also need `docker` and `docker-compose`, and the docker daemon
running.

*Double-check that the revision number in the manifest 
(the value of N in the line `calc_provider:vN`) is the same
as the REVISION number in calc-provider/Makefile* 

Start the registry with `docker-compose up -d`.

From this folder, type `make` to build, and `make run` to run the
wasmcloud host, install the actor, http server capability
provider, and the calculator capability provider.

To perform a calculation, use

`curl localhost:8080/OP/a/b` where OP is one of these operations:

- `add` or `mul`: add or multiply, can take two or more floating point
  values
- `sub`, `div`, `pow`: subtract, divide, or power raise x to the power
  of y). These operations require exactly two operands


