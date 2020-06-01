use crate::typed_array::TypedArray;

/// A reference-counted vector of `u8` that uses Godot's pool allocator.
pub type ByteArray = TypedArray<u8>;

godot_test!(
    test_byte_array_access {
        let mut arr = ByteArray::new();
        for i in 0..8 {
            arr.push(i);
        }

        let original_read = {
            let read = arr.read();
            assert_eq!(&[0, 1, 2, 3, 4, 5, 6, 7], read.as_slice());
            read.clone()
        };

        let mut cow_arr = arr.new_ref();

        {
            let mut write = cow_arr.write();
            assert_eq!(8, write.len());
            for i in write.as_mut_slice() {
                *i *= 2;
            }
        }

        for i in 0..8 {
            assert_eq!(i * 2, cow_arr.get(i as i32));
        }

        // the write shouldn't have affected the original array
        assert_eq!(&[0, 1, 2, 3, 4, 5, 6, 7], original_read.as_slice());
    }
);

godot_test!(
    test_byte_array_debug {
        let mut arr = ByteArray::new();
        for i in 0..8 {
            arr.push(i);
        }

        assert_eq!(format!("{:?}", arr), "[0, 1, 2, 3, 4, 5, 6, 7]");
    }
);
