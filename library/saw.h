#ifndef SAW_H
#define SAW_H

#include <stdint.h>

#define SAW_MIN 5
#define SAW_MAX 50

typedef enum count_direction_e {
    UP,
    DOWN,
} count_direction_t;

uint32_t saw(void);

#endif // SAW_H
