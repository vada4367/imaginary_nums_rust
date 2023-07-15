#ifndef _TYPES_H
#define _TYPES_H

struct __attribute__((__packed__)) Imaginary
{
    double i;
};

struct __attribute__((__packed__)) Complex
{
    double a;
    struct Imaginary b;
};

struct __attribute__((__packed__)) Coord
{
    struct Complex **matrix;
    unsigned char padding[0x10];  /* I have no idea what it was . . . */
};

struct __attribute__((__packed__)) Polar
{
    double s, c;
};

typedef struct Imaginary Imaginary;
typedef struct Complex Complex;
typedef struct Coord Coord;
typedef struct Polar Polar;

#endif /* _TYPES_H */