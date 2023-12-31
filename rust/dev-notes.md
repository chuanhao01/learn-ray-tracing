Notes
- All objects are already bbox implemented

TODOs
- Fix Hittable not implementing Display
  - The reason we need a vec/list wrapper over Vec is because we cannot implement traits over them like Display
- Rc broke because camera.render takes in generic which requires sync (Just needs a single threaded implementation)
- BVH now implements HittableWithBBox (Supposed to?)
- Need to implement Any downcast for the dyn trait
  - So as to allow for checking in test
  - For now just not checking
