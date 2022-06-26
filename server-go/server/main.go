package main

func main() {
	s := NewServer("localhost", 1234)
	s.Run()
}
