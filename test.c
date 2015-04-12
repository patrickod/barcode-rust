#include <libevdev/libevdev.h>
#include <unistd.h>
#include <fcntl.h>
#include <stdio.h>
#include <string.h>
#include <errno.h>

int main() {
  struct libevdev *dev;
  int err;
  int rc = 1;

  dev = libevdev_new();

  int fd = open("/dev/input/event18", O_RDONLY | O_NONBLOCK);

  if (!dev)
    return ENOMEM;
  err = libevdev_set_fd(dev, fd);

  if (err < 0) {
    printf("Failed (errno %d): %s\n", -err, strerror(-err));
    libevdev_free(dev);
  }

  do {
    struct input_event ev;
    rc = libevdev_next_event(dev, LIBEVDEV_READ_FLAG_NORMAL, &ev);
    if (rc == 0)
      printf("Event: %s %s %d\n",
        libevdev_event_type_get_name(ev.type),
        libevdev_event_code_get_name(ev.type, ev.code),
        ev.value);
  } while (rc == 1 || rc == 0 || rc == -EAGAIN);
}
