# spaceform

spaceform is a SIMD-accelerated nath library for 3D graphics.  
It aims to be fast, convenient, as well as flexible. 
It is not tied to any specific API, so you can use for anything related to graphics.

# Speed

Operation             | Time
----------------------|--------
Vector + Vector       | 0.5 ns
Vector - Vector       | 0.5 ns
Vector * Vector       | 0.5 ns
Vector / Vector       | 1.6 ns
Vector abs            | 0.5 ns
Vector horizontal sum | 0.6 ns
Vector min and max    | 0.75 ns
Vector dot product    | 0.9 ns
Vector cross product  | 0.9 ns
Vector * Matrix       | 1.5 ns
Matrix * Matrix       | 5 ns
Matrix transpose      | 1.8 ns
Matrix inverse        | 12 ns

Benchmarked on an i7-4790K @ 4.6 Ghz.
