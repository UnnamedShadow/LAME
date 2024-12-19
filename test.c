#include <stdint.h>
#include <stdlib.h>
#include "lame.h"

Vec_uint8_t hello_world(Vec_uint8_t i) {
    if (i.cap) {
        free(i.ptr);
    }
    Vec_uint8_t o = {
        .ptr = malloc(13),
        .len = 13,
        .cap = 13,
    };
    o.ptr[0] = 'H'
    o.ptr[1] = 'e'
    o.ptr[2] = 'l'
    o.ptr[3] = 'l'
    o.ptr[4] = 'o'
    o.ptr[5] = ','
    o.ptr[6] = ' '
    o.ptr[7] = 'w'
    o.ptr[8] = 'o'
    o.ptr[9] = 'r'
    o.ptr[10] = 'l'
    o.ptr[11] = 'd'
    o.ptr[12] = '!'
    return o;
}
