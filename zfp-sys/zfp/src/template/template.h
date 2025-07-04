#ifndef TEMPLATE_H
#define TEMPLATE_H

/* concatenation */
#define _body(x)       x ## _
#define _cat2(x, y)    x ## _ ## y
#define _cat3(x, y, z) x ## _ ## y ## _ ## z

/* 1- and 2-argument function templates */
#define _t1(function, arg)        _cat2(function, arg)
#define _t2(function, type, dims) _cat3(function, type, dims)

/* 1-argument template instantiation; body must be defined in macro */
#define _tdef1(function, type, args) _cat2(function, type)args _body(function)(type)

#endif
