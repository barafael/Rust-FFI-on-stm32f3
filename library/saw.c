#include "saw.h"

int counter = SAW_MIN;

count_direction_t direction = UP;

int saw() {
    switch (direction) {
        case UP:
            if (counter >= SAW_MAX) {
                direction = DOWN;
                return counter;
            } else {
                return counter++;
            }
            break;
        case DOWN:
            if (counter <= SAW_MIN) {
                direction = UP;
                return counter;
            } else {
                return counter--;
            }
    }
}

