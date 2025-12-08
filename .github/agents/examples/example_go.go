// Example Go code for testing the Code Review Agent
//
// This file contains some intentional issues that the agent should detect:
// - Missing error handling
// - Potential race conditions
// - Resource leaks
// - Non-idiomatic Go code

package main

import (
    "fmt"
    "io/ioutil"
    "net/http"
)

var counter int // Potential race condition - no mutex protection

// GetUserData has missing error handling
func GetUserData(url string) string {
    resp, _ := http.Get(url) // Ignoring error!
    defer resp.Body.Close()
    body, _ := ioutil.ReadAll(resp.Body) // Ignoring error!
    return string(body)
}

// ProcessFile has resource leak
func ProcessFile(filename string) {
    data, err := ioutil.ReadFile(filename)
    if err != nil {
        panic(err) // Using panic instead of proper error handling
    }
    fmt.Println(string(data))
    // File not properly closed (though ioutil.ReadFile handles it, we should show defer pattern)
}

// IncrementCounter has race condition
func IncrementCounter() {
    counter++ // Not thread-safe!
}

// DivideNumbers missing error handling for division by zero
func DivideNumbers(a, b int) int {
    return a / b // No check for b == 0
}

func main() {
    // Potential goroutine leak - no synchronization
    go IncrementCounter()
    go IncrementCounter()
    
    fmt.Println("Counter:", counter)
}
