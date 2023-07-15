/*****************************************************************************
 *                                                                           *
 * COMPLEX_NUMBERS.ELF                                                       *
 *                                                                           *
 * Fully & accuratly decompiled.                                             *
 *                                                                           *
 * SHA-256: EC2B9564B95BB5E41EA7725C2D3333534348029C8790E85DB0296BEE2F5D139E *
 * MD5:     581421051205657BA04E6C80DBF32BB4                                 *
 *                                                                           *
 * WARNING: Original was written in Rust, that's why some builtin functions  *
 * are replaced with libc.                                                   *
 * WARNING: Some of not necessary Rust overloadings are removed.             *
 * WARNING: Some variables & functions are renamed to increase readability.  *
 *                                                                           *
 * The rest of the file is identical to it's original.                       *
 *                                                                           *
 * Enjoy!                                                                    *
 *                                                                           *
******************************************************************************/

#define _GNU_SOURCE

#if defined(_WIN32)
    #define WIN32_LEAN_AND_MEAN
    #include <windows.h>
    #include <tchar.h>
#endif

#include <stdlib.h>
#include <stdio.h>
#include <math.h>
#include <stdbool.h>

#include "types.h"

#define MOND_RES        100
#define MOND_ITERS      100

#define ZOOM_POINTX     -0.75     
#define ZOOM_POINTY     -0.04

#define SQR(x)          ((x) * (x))
#define HALF(x)         ((x) / 2.0)

/* ADDRESS: 0AC00h */
static inline Polar complex_to_polar(const Complex *com) {
    return (Polar) { .c = atan(com->b.i / com->a) * 180.0 / M_PI, .s = sqrt(SQR(com->a) + SQR(com->b.i)) };
}

/* ADDRESS: 0AC90h */
static inline Complex complex_numbers_add(const Complex num1, const Complex num2) {
    return (Complex) { .a = num1.a + num2.a, .b.i = num1.b.i + num2.b.i };
}

/* ADDRESS: 0ADF0h */
static inline Complex polar_to_complex(const Polar *pol) {
    return (Complex) { .a = pol->s * cos(pol->c / 180.0 * M_PI), .b.i = pol->s * sin(pol->c / 180.0 * M_PI) };
}

/* ADDRESS: 0AEC0h */
static inline Polar polar_numbers_mul(const Polar num1, const Polar num2) {
    return (Polar) { .s = num1.s * num2.s, .c = num1.c + num2.c };
}

/* ADDRESS: 0ACE0h */
static Complex complex_numbers_mul(const Complex num1, const Complex num2) {
    Polar mul_polar1 = polar_numbers_mul(complex_to_polar(&num1), complex_to_polar(&num2));
    Complex mul_complex1 = polar_to_complex(&mul_polar1);

    Polar mul_polar2 = polar_numbers_mul(complex_to_polar(&num1), complex_to_polar(&num2));
    Complex mul_complex2 = polar_to_complex(&mul_polar2);

    return (Complex) { .a = mul_complex1.a, .b.i = mul_complex2.b.i };
}

/* ADDRESS: 0A760h */
static bool dot_mond(const Complex *c) {
    Complex num = (Complex){ .a = c->a, .b.i = c->b.i };

    int i = 0;
    for (; i < MOND_ITERS; i ++) {
        num = complex_numbers_add(complex_numbers_mul(num, num), *c);
        if (complex_to_polar(&num).s > 2.0)
            break;
    }

    return i >= MOND_ITERS - 2;
}

size_t _matrix_size_global;
size_t _complex_vector_size_global;

/* ADDRESS: 0A400h */
static void init_mond(Coord *self, double x1, double y1, double x2, double y2, int n) {
    Complex **matrix = NULL;
    size_t matrix_size = 0ULL;
    int i, j;

    i = 0;
    while (++i < n) {
        Complex *complex_vector = NULL;
        size_t complex_vector_size = 0ULL;

        j = 0;
        while (++j < n) {
            Complex value;
            value.a   = x1 + (x2 - x1) / (double)n * (double)i;
            value.b.i = y1 + (y2 - y1) / (double)n * (double)j;
            
            complex_vector = realloc(complex_vector, sizeof(Complex) * ++complex_vector_size);
            complex_vector[complex_vector_size - 1] = value;
        }

        _complex_vector_size_global = complex_vector_size;
        matrix = realloc(matrix, sizeof(Complex *) * ++matrix_size);
        matrix[matrix_size - 1] = complex_vector;
    }

    _matrix_size_global = matrix_size;
    self->matrix = matrix;
    return;
}

size_t _bool_screen_size_global;
size_t _bool_vector_size_global;

/* ADDRESS: 0A890h */
static bool **spawn_mond(Coord *coords) {
    bool **bool_screen = NULL;
    size_t bool_screen_size = 0ULL;
    int i, j;

    i = 0;
    while (++i < _matrix_size_global) {
        bool *bool_vector = NULL;
        size_t bool_vector_size = 0ULL;

        j = 0;
        while (++j < _complex_vector_size_global - 1) {
            Complex *c_vector = coords->matrix[i];
            Complex c_value = c_vector[j];
            
            bool_vector = realloc(bool_vector, sizeof(bool) * ++bool_vector_size);
            bool_vector[bool_vector_size - 1] = dot_mond(&c_value) & 1;
        }

        _bool_vector_size_global = bool_vector_size;
        bool_screen = realloc(bool_screen, sizeof(bool *) * ++bool_screen_size);
        bool_screen[bool_screen_size - 1] = bool_vector;
    }

    _bool_screen_size_global = bool_screen_size;
    return bool_screen;
}

/* ADDRESS: 8920h */
static void print_mond(bool **screen_arr) {
    int i, j;

    i = 0;
    while (++i < _bool_vector_size_global - 1) {
        
        j = 0;
        while (++j < _bool_screen_size_global) {
            bool *row = screen_arr[j];
            fprintf(stdout, "%.2s", "  ##" + 2 * row[i]);
        }

        fputc('\n', stdout);
    }
    return;
}

/* ADDRESS: 8AF0h */
int main(void) {
    Coord coords = { 0 };
    double zoom = 2.0;

    for (;;) {
        #if defined(_WIN32)
            Sleep(10);
            SetConsoleCursorPosition(GetStdHandle(STD_OUTPUT_HANDLE), (COORD){0,0});
        #endif
    
        zoom /= 1.01;
        init_mond (
            &coords,
            ZOOM_POINTX - HALF(zoom),
            ZOOM_POINTY - HALF(zoom),
            HALF(zoom) + ZOOM_POINTX,
            HALF(zoom) + ZOOM_POINTY,
            MOND_RES
        );
 
        bool **screen_arr = spawn_mond(&coords);
        print_mond(screen_arr);

        for (size_t i = 0; i < _matrix_size_global; i++)
            free(coords.matrix[i]);
        free(coords.matrix);

        for (size_t i = 0; i < _bool_screen_size_global; i++)
            free(screen_arr[i]);
        free(screen_arr);
    }

    return 0;
}