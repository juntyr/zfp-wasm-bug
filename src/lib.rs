#[cfg(test)]
mod tests {
    use numcodecs::StaticCodecVersion;
    use numcodecs_zfp::ZfpCompressionMode;

    #[test]
    fn test_me() {
        use ndarray::Array;
        use ndarray_rand::RandomExt;
        use ndarray_rand::rand_distr::Normal;
        use numcodecs::Codec;
        use numcodecs_zfp::ZfpCodec;

        let data = Array::random((1, 1, 1, 9900, 16987), Normal::new(0., 1.).unwrap());

        for codec in [
            ZfpCodec {
                mode: ZfpCompressionMode::FixedAccuracy { tolerance: 4.0 },
                version: StaticCodecVersion,
            },
            ZfpCodec {
                mode: ZfpCompressionMode::FixedAccuracy { tolerance: 0.4 },
                version: StaticCodecVersion,
            },
            ZfpCodec {
                mode: ZfpCompressionMode::FixedAccuracy { tolerance: 0.04 },
                version: StaticCodecVersion,
            },
        ] {
            eprintln!("start encode");
            let encoded = codec
                .encode(numcodecs::AnyArrayView::F32(data.view().into_dyn()).cow())
                .expect("encode");
            eprintln!("start decode");
            let _decoded = codec.decode(encoded.into_cow()).expect("decode");
        }
    }
}
