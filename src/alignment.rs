#[test]
fn alignment() {
    use std::mem;

    // * Alignment: what addresses are valid to store the value at.
    //              power of 2.
    //              any value of this type must be stored in a muliple of alignment.
    //
    // * Size: offset in bytes successive elements in an array (includes alignment padding).
    //         multiple of alignment.
    //
    // https://doc.rust-lang.org/reference/type-layout.html

    #[repr(C)]
    struct Faa {
        first: u8,
        second: u16,
        third: u8,
    }

    assert_eq!(mem::offset_of!(Faa, first), 0);
    assert_eq!(mem::offset_of!(Faa, second), 2);
    assert_eq!(mem::offset_of!(Faa, third), 4);
    assert_eq!(mem::align_of::<Faa>(), 2);
    assert_eq!(mem::size_of::<Faa>(), 6);

    #[repr(C)]
    struct Foo {
        a: u8,
        b: u8,
        c: u16,
    }

    assert_eq!(mem::offset_of!(Foo, a), 0);
    assert_eq!(mem::offset_of!(Foo, b), 1);
    assert_eq!(mem::offset_of!(Foo, c), 2);
    assert_eq!(mem::align_of::<Foo>(), 2);
    assert_eq!(mem::size_of::<Foo>(), 4);

    #[repr(C)]
    struct Bar {
        first: u32,
        second: u32,
        third: u8,
    }
    assert_eq!(mem::align_of::<Bar>(), 4);
    assert_eq!(mem::size_of::<Bar>(), 12);

    #[repr(C)]
    struct Baz {
        first: u32,
        second: u32,
        third: i64,
    }
    assert_eq!(mem::align_of::<Baz>(), 8);
    assert_eq!(mem::size_of::<Baz>(), 16);

    struct A1 {
        a: u8,
        b: u8,
        c: u16,
    }

    struct A2 {
        a: u8,
        b: u16,
        c: u8,
    }

    assert_eq!(mem::size_of::<A1>(), 4);
    assert_eq!(mem::size_of::<A1>(), mem::size_of::<A2>());

    // Representations `#[repr(..)]` on struct, enum and union:
    // * rust (default)
    // * C
    // * primitive (enums only)
    // * transparent

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[repr(transparent)]
    struct Newtype {
        a: u8,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[repr(transparent)]
    struct Wrapper<T> {
        value: T,
    }

    let n = Wrapper {
        value: Newtype { a: 144 },
    };
    let x: u8 = unsafe { std::mem::transmute(n) };
    assert_eq!(x, 144);
    let n2: Wrapper<Newtype> = unsafe { std::mem::transmute(n) };
    assert_eq!(n2, n);
}

#[test]
fn alignment_modifiers() {
    // The alignment can be increased or decreased:
    // * `align`: increase
    // * `packed`: decrease

    struct A {
        a1: u8,
        a2: u8,
    }
    assert_eq!(std::mem::align_of::<A>(), 1);

    #[repr(C, align(2))]
    struct B {
        b1: u8,
        b2: u8,
    }
    assert_eq!(std::mem::align_of::<B>(), 2);

    #[repr(packed(1))]
    struct C {
        b1: u8,
        b2: u16,
    }
    assert_eq!(std::mem::align_of::<C>(), 1);
}

#[test]
fn nested_struct() {
    use std::mem;

    struct A {
        a1: u16,
        a2: u8,
    }

    struct B {
        b1: u8,
        b2: u8,
    }

    struct C {
        c1: A,
        c2: B,
        c3: u8,
    }

    assert_eq!(mem::offset_of!(C, c1), 0);
    assert_eq!(mem::offset_of!(C, c2), 4);
    assert_eq!(mem::offset_of!(C, c3), 6);
    assert_eq!(mem::align_of::<C>(), 2);
    assert_eq!(mem::size_of::<C>(), 8);
}

#[test]
fn nested_struct_c() {
    use std::mem;

    #[repr(C)]
    struct A {
        a1: u8,
        a2: u16,
    }

    #[repr(C)]
    struct B {
        b1: u8,
        b2: u8,
    }

    #[repr(C)]
    struct C {
        c1: A,
        c2: u8,
        c3: B,
    }

    assert_eq!(mem::offset_of!(C, c1), 0);
    assert_eq!(mem::offset_of!(C, c2), 4);
    assert_eq!(mem::offset_of!(C, c3), 5);
    assert_eq!(mem::align_of::<C>(), 2);
    assert_eq!(mem::size_of::<C>(), 8);
}

#[test]
fn alignment_algo() {
    use std::mem;

    /*
    fn padding_needed_for(offset: usize, alignment: usize) -> usize {
        let misalignment = offset % alignment;
        if misalignment > 0 {
            alignment - misalignment
        } else {
            0
        }
    }

    struct.alignment = struct.fields().map(|field| field.alignment).max();

    let current_offset = 0;

    for (field, idx) in struct.fields_in_declaration_order() {
        if idx != 0 {
            current_offset += padding_needed_for(current_offset, field.alignment);
        }

        struct[field].offset = current_offset;

        current_offset += field.size;
    }

    struct.size = current_offset + padding_needed_for(current_offset, struct.alignment);
    */

    #[repr(C)]
    struct A {
        a1: u32,
        a2: u8,
    }
    assert_eq!(mem::align_of::<A>(), 4);
    assert_eq!(mem::size_of::<A>(), 8);
}
