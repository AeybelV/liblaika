#include <liblaika/liblaika.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>

int main() {
  // Initialize the logger
  const char *log_dir = "logs";
  size_t ecc_len = 8;
  LibLaikaLogger *logger = liblaika_initialize_logger(log_dir, ecc_len);
  if (logger == NULL) {
    fprintf(stderr, "Failed to initialize logger\n");
    return 1;
  }

  // Log a message
  const char *message = "Hello from C!";
  if (!liblaika_log(logger, message)) {
    fprintf(stderr, "Failed to log message\n");
  }

  // Clean up
  liblaika_destroy_logger(logger);

  return 0;
}
