package main

import (
	"context"
	"fmt"

	"github.com/fastly/compute-sdk-go/fsthttp"
)


func fibonacci(n int) int {
	if n < 2 {
		return n
	}
	return fibonacci(n-1) + fibonacci(n-2)
}

func main() {
	fsthttp.ServeFunc(func(ctx context.Context, w fsthttp.ResponseWriter, r *fsthttp.Request) {

		result := fibonacci(40)

		fmt.Fprintf(w, "result:           %d\n", result)
		if r.URL.RawQuery != "" {
			fmt.Fprintf(w, "params:           %s\n", r.URL.RawQuery)
		}
	})
}