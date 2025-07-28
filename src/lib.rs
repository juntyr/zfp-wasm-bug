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

        for max_prec in [30, 25, 20, 15, 10, 5, 4, 3, 2, 1] {
            eprintln!("start encode");
            let encoded = ZfpCodec {
                mode: ZfpCompressionMode::Expert { min_bits=0, max_bits=0, max_prec=max_prec, min_exp=-1075 },
                version: StaticCodecVersion,
            }
                .encode(numcodecs::AnyArrayView::F32(data.view().into_dyn()).cow())
                .expect("encode");
            eprintln!("start decode");
            let _decoded = codec.decode(encoded.into_cow()).expect("decode");
        }
    }
}
