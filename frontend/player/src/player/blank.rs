use bytes::Bytes;
use h264::AVCDecoderConfigurationRecord;
use mp4::{
    types::{
        avc1::Avc1,
        avcc::AvcC,
        hdlr::{HandlerType, Hdlr},
        mdat::Mdat,
        mdhd::Mdhd,
        mdia::Mdia,
        mfhd::Mfhd,
        minf::Minf,
        moof::Moof,
        moov::Moov,
        mvex::Mvex,
        mvhd::Mvhd,
        stbl::Stbl,
        stco::Stco,
        stsc::Stsc,
        stsd::{SampleEntry, Stsd, VisualSampleEntry},
        stsz::Stsz,
        stts::Stts,
        tfdt::Tfdt,
        tfhd::Tfhd,
        tkhd::Tkhd,
        traf::Traf,
        trak::Trak,
        trex::Trex,
        trun::{Trun, TrunSample, TrunSampleFlag},
        vmhd::Vmhd,
    },
    BoxType, DynBox,
};

pub struct VideoFactory {
    timescale: u32,
    sequence_number: u32,
    stop_at: Option<f64>,
}

impl VideoFactory {
    pub fn new(timescale: u32) -> Self {
        Self {
            sequence_number: 0,
            timescale,
            stop_at: None,
        }
    }

    pub fn stop_at(&self) -> Option<f64> {
        self.stop_at
    }

    pub fn set_stop_at(&mut self, stop_at: Option<f64>) {
        self.stop_at = stop_at;
    }

    pub fn moov(&self) -> Moov {
        Moov::new(
            Mvhd::new(0, 0, 1000, 0, 2),
            vec![Trak::new(
                Tkhd::new(
                    0,
                    0,
                    1,
                    0,
                    Some((100, 100)),
                ),
                None,
                Mdia::new(
                    Mdhd::new(0, 0, self.timescale, 0),
                    Hdlr::new(HandlerType::Vide, "".to_string()),
                    Minf::new(
                        Stbl::new(
                            Stsd::new(vec![DynBox::Avc1(Avc1::new(
                                SampleEntry::new(VisualSampleEntry::new(2, 2, None)),
                                AvcC::new(AVCDecoderConfigurationRecord {
                                    configuration_version: 1,
                                    profile_indication: 100,
                                    profile_compatibility: 0,
                                    level_indication: 10,
                                    length_size_minus_one: 3,
                                    sps: vec![Bytes::from_static(b"\x67\x64\x00\x0a\xac\xd9\x47\x3f\x9e\x7c\x04\x40\x00\x00\x03\x00\x40\x00\x00\x03\x00\x83\xc4\x89\x65\x80")],
                                    pps: vec![Bytes::from_static(b"\x68\xeb\xe3\xcb\x22\xc0")],
                                    extended_config: None,
                                }),
                                None,
                            ))]),
                            Stts::new(vec![]),
                            Stsc::new(vec![]),
                            Stco::new(vec![]),
                            Some(Stsz::new(0, vec![])),
                        ),
                        Some(Vmhd::new()),
                        None,
                    ),
                )
            )],
            Some(Mvex::new(vec![Trex::new(1)], None)),
        )
    }

    pub fn moof_mdat(&mut self, decode_time: u64, duration: u32) -> (Moof, Mdat) {
        self.sequence_number += 1;

        // This is a dummy mdat which represents a single frame of video.
        // The frame is 100x100 pixels h264 and its entirely black.
        // The reason we do this is because we need to be able to generate blank frames
        // On the client side. So that if a client selects audio only, we can still
        // provide the video decoder with some blank frames to keep it happy.

        let mdat = Mdat::new(vec![Bytes::from_static(b"\x00\x00\x02\xad\x06\x05\xff\xff\xa9\xdc\x45\xe9\xbd\xe6\xd9\x48\xb7\x96\x2c\xd8\x20\xd9\x23\xee\xef\x78\x32\x36\x34\x20\x2d\x20\x63\x6f\x72\x65\x20\x31\x36\x33\x20\x72\x33\x30\x36\x30\x20\x35\x64\x62\x36\x61\x61\x36\x20\x2d\x20\x48\x2e\x32\x36\x34\x2f\x4d\x50\x45\x47\x2d\x34\x20\x41\x56\x43\x20\x63\x6f\x64\x65\x63\x20\x2d\x20\x43\x6f\x70\x79\x6c\x65\x66\x74\x20\x32\x30\x30\x33\x2d\x32\x30\x32\x31\x20\x2d\x20\x68\x74\x74\x70\x3a\x2f\x2f\x77\x77\x77\x2e\x76\x69\x64\x65\x6f\x6c\x61\x6e\x2e\x6f\x72\x67\x2f\x78\x32\x36\x34\x2e\x68\x74\x6d\x6c\x20\x2d\x20\x6f\x70\x74\x69\x6f\x6e\x73\x3a\x20\x63\x61\x62\x61\x63\x3d\x31\x20\x72\x65\x66\x3d\x33\x20\x64\x65\x62\x6c\x6f\x63\x6b\x3d\x31\x3a\x30\x3a\x30\x20\x61\x6e\x61\x6c\x79\x73\x65\x3d\x30\x78\x33\x3a\x30\x78\x31\x31\x33\x20\x6d\x65\x3d\x68\x65\x78\x20\x73\x75\x62\x6d\x65\x3d\x37\x20\x70\x73\x79\x3d\x31\x20\x70\x73\x79\x5f\x72\x64\x3d\x31\x2e\x30\x30\x3a\x30\x2e\x30\x30\x20\x6d\x69\x78\x65\x64\x5f\x72\x65\x66\x3d\x31\x20\x6d\x65\x5f\x72\x61\x6e\x67\x65\x3d\x31\x36\x20\x63\x68\x72\x6f\x6d\x61\x5f\x6d\x65\x3d\x31\x20\x74\x72\x65\x6c\x6c\x69\x73\x3d\x31\x20\x38\x78\x38\x64\x63\x74\x3d\x31\x20\x63\x71\x6d\x3d\x30\x20\x64\x65\x61\x64\x7a\x6f\x6e\x65\x3d\x32\x31\x2c\x31\x31\x20\x66\x61\x73\x74\x5f\x70\x73\x6b\x69\x70\x3d\x31\x20\x63\x68\x72\x6f\x6d\x61\x5f\x71\x70\x5f\x6f\x66\x66\x73\x65\x74\x3d\x2d\x32\x20\x74\x68\x72\x65\x61\x64\x73\x3d\x33\x20\x6c\x6f\x6f\x6b\x61\x68\x65\x61\x64\x5f\x74\x68\x72\x65\x61\x64\x73\x3d\x31\x20\x73\x6c\x69\x63\x65\x64\x5f\x74\x68\x72\x65\x61\x64\x73\x3d\x30\x20\x6e\x72\x3d\x30\x20\x64\x65\x63\x69\x6d\x61\x74\x65\x3d\x31\x20\x69\x6e\x74\x65\x72\x6c\x61\x63\x65\x64\x3d\x30\x20\x62\x6c\x75\x72\x61\x79\x5f\x63\x6f\x6d\x70\x61\x74\x3d\x30\x20\x63\x6f\x6e\x73\x74\x72\x61\x69\x6e\x65\x64\x5f\x69\x6e\x74\x72\x61\x3d\x30\x20\x62\x66\x72\x61\x6d\x65\x73\x3d\x33\x20\x62\x5f\x70\x79\x72\x61\x6d\x69\x64\x3d\x32\x20\x62\x5f\x61\x64\x61\x70\x74\x3d\x31\x20\x62\x5f\x62\x69\x61\x73\x3d\x30\x20\x64\x69\x72\x65\x63\x74\x3d\x31\x20\x77\x65\x69\x67\x68\x74\x62\x3d\x31\x20\x6f\x70\x65\x6e\x5f\x67\x6f\x70\x3d\x30\x20\x77\x65\x69\x67\x68\x74\x70\x3d\x32\x20\x6b\x65\x79\x69\x6e\x74\x3d\x32\x35\x30\x20\x6b\x65\x79\x69\x6e\x74\x5f\x6d\x69\x6e\x3d\x31\x20\x73\x63\x65\x6e\x65\x63\x75\x74\x3d\x34\x30\x20\x69\x6e\x74\x72\x61\x5f\x72\x65\x66\x72\x65\x73\x68\x3d\x30\x20\x72\x63\x5f\x6c\x6f\x6f\x6b\x61\x68\x65\x61\x64\x3d\x34\x30\x20\x72\x63\x3d\x63\x72\x66\x20\x6d\x62\x74\x72\x65\x65\x3d\x31\x20\x63\x72\x66\x3d\x32\x33\x2e\x30\x20\x71\x63\x6f\x6d\x70\x3d\x30\x2e\x36\x30\x20\x71\x70\x6d\x69\x6e\x3d\x30\x20\x71\x70\x6d\x61\x78\x3d\x36\x39\x20\x71\x70\x73\x74\x65\x70\x3d\x34\x20\x69\x70\x5f\x72\x61\x74\x69\x6f\x3d\x31\x2e\x34\x30\x20\x61\x71\x3d\x31\x3a\x31\x2e\x30\x30\x00\x80\x00\x00\x00\x2e\x65\x88\x84\x00\x15\xff\xfe\xf7\xc9\xef\xc0\xa6\xeb\xdb\xde\xb5\xbf\x93\xcf\x48\xfc\x2c\xb7\x3e\xca\xf4\x4d\xb5\x40\x78\x78\xd4\x35\xda\xfb\xf5\xfb\x25\x80\x10\xa0\x06\xb8\x55\x69\xc1")]);

        let mut moof = Moof::new(
            Mfhd::new(self.sequence_number),
            vec![Traf::new(
                Tfhd::new(
                    1,
                    None,
                    None,
                    Some(duration),
                    Some(mdat.primitive_size() as u32),
                    Some(TrunSampleFlag {
                        reserved: 0,
                        is_leading: 0,
                        sample_depends_on: 2,
                        sample_is_depended_on: 0,
                        sample_has_redundancy: 0,
                        sample_padding_value: 0,
                        sample_is_non_sync_sample: false,
                        sample_degradation_priority: 0,
                    }),
                ),
                Some(Trun::new(
                    vec![TrunSample {
                        composition_time_offset: None,
                        duration: None,
                        size: None,
                        flags: None,
                    }],
                    None,
                )),
                Some(Tfdt::new(decode_time)),
            )],
        );

        let offset = moof.size() + 8;

        moof.traf[0].trun.as_mut().unwrap().data_offset = Some(offset as i32);

        (moof, mdat)
    }
}
