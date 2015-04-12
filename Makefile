all:
	gcc -g -o test test.c `pkg-config --libs --cflags libevdev`

clean:
	rm test
