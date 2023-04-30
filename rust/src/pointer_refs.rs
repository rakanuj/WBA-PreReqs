/**
 * references pointers - point to a resource in memory
 */
pub fn run() {
     // primitive array
     let arr1 = [1,2,3];
     let arr2 = arr1;
     println!("Array: {:?}", (arr1, arr2));
     
     // with non-primitives, the first variable will no longer
     // hold that value.  you'll need to use a reference(&) to point 
     // to the resource.
     let vec1 = vec![1,2,3];
     let vec2 = &vec1;
     println!("Vector: {:?}", (&vec1, vec2));
}