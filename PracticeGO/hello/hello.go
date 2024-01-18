package main

import (
	"fmt"
	"log"

	"example.com/greetings"
)

func main() {
	log.SetPrefix("greetings: ")
	log.SetFlags(0)
	// Get a greeting message and print it.
	message, _ := greetings.Hello("Manish")
	fmt.Println(message)

	_, err := greetings.Hello("")
	if err != nil {
		log.Println(err)
	}

	names := []string{"Manish", "Rajesh", "Ramesh"}
	messages, err := greetings.Hellos(names)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println(messages)
}
