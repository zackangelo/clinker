# Clinker

A Consul Connect control plane for Linkerd 2.

### Motivation

I don't know, this is probably crazy. TBD.

### Project Layout

`clinker-gen` Generated code that describe the destination API that linkerd data plane nodes use to make traffic routing decisions.

`clinker-server` Control plane API server that linkerd data plane nodes connect to.

`clinker-consul` A Consul HTTP API client. Only implements what's necessary for now.