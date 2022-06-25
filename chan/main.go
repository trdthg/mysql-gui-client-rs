package main

func main() {
	ch := make(chan int, 1)
	ch <- 1
	d := <-ch
	print(d)
}
