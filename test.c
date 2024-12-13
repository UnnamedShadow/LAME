#include <stdint.h>
#include <stdlib.h>
#include "lame.h"

Vec_uint8_t hello_world(Vec_uint8_t i) {
    free(i.ptr);
    Vec_uint8_t o = {
        .ptr = malloc(13),
        .len = 13,
        .cap = 13,
    };
    for (int j = 0; j < 13; j++) {
        if (j == 0) {
            o.ptr[j] = 'H';
        } else if (j == 1) {
            o.ptr[j] = 'e';
        } else if (j == 2) {
            o.ptr[j] = 'l';
        } else if (j == 3) {
            o.ptr[j] = 'l';
        } else if (j == 4) {
            o.ptr[j] = 'o';
        } else if (j == 5) {
            o.ptr[j] = ',';
        } else if (j == 6) {
            o.ptr[j] = ' ';
        } else if (j == 7) {
            o.ptr[j] = 'w';
        } else if (j == 8) {
            o.ptr[j] = 'o';
        } else if (j == 9) {
            o.ptr[j] = 'r';
        } else if (j == 10) {
            o.ptr[j] = 'l';
        } else if (j == 11) {
            o.ptr[j] = 'd';
        } else if (j == 12) {
            o.ptr[j] = '!';
        }
    }
    return o;
}
