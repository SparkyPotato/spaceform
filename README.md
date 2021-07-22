# spaceform

spaceform is a SIMD-accelerated nath library for 3D graphics.  
It aims to be fast, convenient, as well as flexible. 
It is not tied to any specific API, so you can use for anything related to graphics.

## Speed

Operation             | cgmath  | spaceform simd | spaceform scalar
----------------------|---------|----------------|-----------------
Vector + Vector       | 0.45 ns | 0.23 ns        | 0.45 ns
Vector - Vector       | 0.45 ns | 0.23 ns        | 0.45 ns
Vector * Vector       | 0.45 ns | 0.23 ns        | 0.45 ns
Vector / Vector       | 0.45 ns | 0.23 ns        | 0.45 ns
Vector abs            | -       | 0.23 ns        | 0.45 ns
Vector horizontal sum | 0.22 ns | 0.22 ns        | 0.22 ns
Vector min and max    | -       | 0.23 ns        | 0.45 ns
Vector dot product    | 0.23 ns | 0.23 ns        | 0.23 ns
Vector cross product  | 0.46 ns | 0.23 ns        | 0.45 ns
Vector * Matrix       | 5.5 ns  | 0.22 ns        | 0.45 ns
Matrix * Matrix       | 24 ns   | 0.95 ns        | 2 ns
Matrix transpose      | 2 ns    | 0.9 ns         | 2 ns
Matrix inverse        | 12 ns   | 0.95 ns        | 2 ns

Benchmarked on an i7-4790K @ 4.6 Ghz.  
At worst, spaceform is as fast as cgmath.  
For vector-vector operations, spaceform is around twice as fast as cgmath.  
For matrix operations, spaceform blows cgmath out of the water.  
In some cases, even scalar spaceform is faster than cgmath.
