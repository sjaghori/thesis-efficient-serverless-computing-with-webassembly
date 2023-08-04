/// <reference types="@fastly/js-compute" />

addEventListener("fetch", (event) => event.respondWith(handleRequest(event)));

function fibonacci(n) {
  if (n < 2) {
    return n;
  } else {
    return fibonacci(n - 1) + fibonacci(n - 2);
  }
}

async function handleRequest(event) {
  // Get the client request.
  let req = event.request;

  let url = new URL(req.url);

  // If request is to the `/` path...
  if (url.pathname == "/") {
    const result = fibonacci(40);

    return new Response(
      JSON.stringify({
        result,
        params: url.searchParams,
      }),
      {
        status: 200,
        headers: new Headers({ "Content-Type": "application/json" }),
      }
    );
  }

  // Catch all other requests and return a 404.
  return new Response("The page you requested could not be found", {
    status: 404,
  });
}
