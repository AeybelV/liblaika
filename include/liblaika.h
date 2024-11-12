#ifndef LIBLAIKA_H
#define LIBLAIKA_H

#include <stddef.h>
// Define Logger as an opaque struct
typedef struct LibLaikaLogger LibLaikaLogger;

extern void liblaika_hello();
extern LibLaikaLogger *liblaika_initialize_logger(const char *dir_path,
                                                  size_t ecc_len);
extern int liblaika_log(LibLaikaLogger *logger, const char *message);
// TODO: Add verify_and_repair_log ?
extern void liblaika_destroy_logger(LibLaikaLogger *logger);

#endif // LIBLAIKA_H
