use nih_plug::prelude::*;
use nih_plug_iced::IcedState;

use crate::buffers::*;

use std::sync::Arc;

mod buffers;
mod editor;
mod proc;

const FREQ_RESOLUTION: f32 = 3.0;
static TEMPORAL_AVG_DEPTH: usize = 8;

pub struct AT {
    params: Arc<ATparams>,
    buffers: buffers::ATbuffers,
    results: Arc<Vec<TFresult>>,
    s_r: f32,
}

#[derive(Params)]
struct ATparams {
    #[persist = "editor-state"]
    editor_state: Arc<IcedState>,
}

impl Default for AT {
    fn default() -> Self {
        Self {
            params: Arc::new(ATparams::default()),
            buffers: ATbuffers::default(),
            results: Vec::new().into(),
            s_r: 0.0,
        }
    }
}

impl Default for ATparams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),
        }
    }
}

impl Plugin for AT {
    //fn editor(&self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {}

    fn initialize(
        &mut self,
        audio_config: &AudioIOLayout,
        buffer_config: &BufferConfig,
        context: &mut impl InitContext<Self>,
    ) -> bool {
        self.s_r = buffer_config.sample_rate;
        let n_chan: std::num::NonZeroU32 = audio_config.main_input_channels.unwrap();
        self.buffers.init(self.s_r as usize, n_chan.get() as usize);

        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        // probably missing some event handling here: need to watch for buffer resets from user
        self.buffers.update(&buffer);

        ProcessStatus::Normal
    }

    const NAME: &'static str = "Alignment Tool v.1";
    const VENDOR: &'static str = "Wirebender Audio";
    const URL: &'static str = "";
    const EMAIL: &'static str = "";

    const VERSION: &'static str = "0.1.0";

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(3),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(3),
            main_output_channels: NonZeroU32::new(3),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(4),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(4),
            main_output_channels: NonZeroU32::new(4),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(5),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(5),
            main_output_channels: NonZeroU32::new(5),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(6),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(6),
            main_output_channels: NonZeroU32::new(6),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(7),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(7),
            main_output_channels: NonZeroU32::new(7),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(8),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(8),
            main_output_channels: NonZeroU32::new(8),
            ..AudioIOLayout::const_default()
        },
    ];

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }
}

impl ClapPlugin for AT {
    const CLAP_ID: &'static str = "discrete.symbol.continuous.syntax.alignment.tool.0.1";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("For rapid sound system deployment");
    const CLAP_MANUAL_URL: Option<&'static str> = None;
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::Utility];
}

impl Vst3Plugin for AT {
    const VST3_CLASS_ID: [u8; 16] = *b"AlignmentToolv.1";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[Vst3SubCategory::Tools];
}

nih_export_clap!(AT);
nih_export_vst3!(AT);
