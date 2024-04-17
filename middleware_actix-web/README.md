# Middleware concept

## What is middleware?

Middleware is a function that is called before the main function. It is used to modify the request and response objects. It is used to perform tasks like parsing the request, authenticating the user, etc.

## How to use middleware in Actix-web?

Actix Web's middleware system allows us to add additional behavior to request/response processing. Middleware can hook into an incoming request process, enabling us to modify requests as well as halt request processing to return a response early.

Middleware can also hook into response processing.

Typically, middleware is involved in the following actions:

- Pre-process the Request
- Post-process a Response
- Modify application state
- Access external services (redis, logging, sessions)

Middleware is registered for each `App`, `scope`, or `Resource` and executed in opposite order as registration. In general, a middleware is a type that implements the `Service` trait and `Transform` trait. Each method in the traits has a default implementation. Each method can return a result immediately or a future object.
