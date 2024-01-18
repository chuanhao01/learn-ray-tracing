# Learn Raytracing

This goal of this repo is 2 parts:

- Make a good raytracer to learn and see what computer graphics is all about.
- Use this project as a benchamrk to learn new programming languages.

The inspiration for starting this project was from a senior introducitng [RayTracingInOneWeekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) to me. The topic of computer graphics also seems to combine the topics of math and computer science well and produces (in my opinion) pretty cool results.

For anyone wondering if their should/could do this project, here is my 2cents(mainly for the [`cpp`](./cpp) based implementation):

- You do need to understand the math presented in the book to some extent, as figuring out bugs or even following along requires to "get" what the math is accomplishing
  - I.e. You do not need to understand or derive the proof for solving the "t" value when a ray intersects a sphere, but you do need to understand that with the "t" value you get the point in which the ray intersects the sphere (Which is then used in later parts)
- Coming from a high level programming language (Python, JS), there are parts in the code where I did not get why certain `cpp` syntax was used/why certain parts were implemented a certain way.
  - I did not really try to implement them another way (I.e. Using a constant reference to a `Point3D` object instead of a pointer)
- Things like linking the header files and building the project is also not mentioned in the book
  - This is not a big issue, but I had hoped to be given a specific example on a platform so I had a starting place to goolge from
- It is a really cool and fun project that you can do with almost no outside dependencies (Really nice just having to only think about developing the code and not worrying about dependency hell for a change)

Here are some pretty renders :D, taken from the [`rust`](./rust) implementation.  

<div align="center">

![Weekend Final Render](./rust/images/weekend.png)  
Weekend Final Render  

![Week Final Render](./rust/images/week.png)  
Week Final Render (Without perlin noise and gas demos)  

![Cornell Box](./rust/images/cornell-box.png)  
Cornell Box  

</div>


## Code

You can find a quick summary of the raytracers in their language below:

- [`cpp`](./cpp)
  - Follows really closely with [RayTracingInOneWeekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
  - Only really added the `Cmake` build system as well as splitting up the header files more
- [`rust`](./rust)
  - Really enjoyed the developer tooling and working with `rust` more
  - Also noticed it ran faster than the `cpp` implementation sometimes
- More to come soon...
