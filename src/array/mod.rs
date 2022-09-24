// // Creates an array[u8] of elements split into groups the length of size. If array can't be split evenly, the final chunk will be the remaining elements.
// #[inline]
// pub fn chunk<T>(arr: &[T], size: usize) -> &[T] {
//     let length = arr.len();
//     if !length < 0 || size < 1 {
//         return &[]
//     }
//     let index = 0;
//     let mut resIndex = 0;
//     const result = [];
//     // length.ceil(size)

//     while index < length {

//         result[resIndex] = slice(array, index, (index += size));
//         resIndex += 1;
//         index += size;
//     }
//     return result
// }

// #[test]
// fn test_chunk() {
//     let a = chunk(&[1, 2, 3, 4], 2); // => [[1, 2],3, 4]]

//     let b = chunk(&[1, 2, 3, 4], 3); // => [[1, 2, 3], [4]]

//     assert_eq!(a, [[1, 2],[3, 4]]);
//     assert_eq!(b, [[1, 2, 3], [4]]);
// }
