#[test]
fn zerocopy() {
    use zerocopy::*;

    #[derive(FromZeros, IntoBytes, Immutable)]
    #[repr(C)]
    struct PacketHeader {
        src_port: [u8; 2],
        dst_port: [u8; 2],
        length: [u8; 2],
        checksum: [u8; 2],
    }

    let mut header = PacketHeader {
        src_port: [0, 1],
        dst_port: [2, 3],
        length: [4, 5],
        checksum: [6, 7],
    };

    let bytes = header.as_bytes();
    assert_eq!(bytes, [0, 1, 2, 3, 4, 5, 6, 7]);

    header.zero();

    assert_eq!(header.src_port, [0, 0]);
    assert_eq!(header.dst_port, [0, 0]);
    assert_eq!(header.length, [0, 0]);
    assert_eq!(header.checksum, [0, 0]);
}

#[test]
fn zerocopy_custom_type() {
    use zerocopy::byteorder::network_endian::U16;
    use zerocopy::*;

    #[derive(FromZeros, IntoBytes, Immutable)]
    #[repr(C)]
    struct Test {
        src_port: u8,
        length: Length,
        checksum: [u8; 2],
    }

    #[derive(FromZeros, IntoBytes, Immutable)]
    #[repr(C)]
    struct Length {
        factor: u8,
        value: U16,
    }

    let header = Test {
        src_port: 1u8,
        length: Length {
            factor: 2u8,
            value: U16::new(0xFF00),
        },
        checksum: [4, 5],
    };
    let bytes = header.as_bytes();
    assert_eq!(bytes, [1, 2, 255, 0, 4, 5]);
}
