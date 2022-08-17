// Refer to nih-plug's examples for information about what's going on here.

use nih_plug::prelude::*;
use std::sync::Arc;

#[derive(Default)]
struct PhaseInverter {
    params: Arc<PhaseInverterParams>
}

#[derive(Default, Params)]
struct PhaseInverterParams {}

impl Plugin for PhaseInverter {
    const NAME: &'static str = "Phase Inverter";
    const VENDOR: &'static str = "Tonogram";
    const URL: &'static str = "https://tonogr.am/";
    const EMAIL: &'static str = "reilly@tonogr.am";

    const VERSION: &'static str = "1.0.0";

    const DEFAULT_NUM_INPUTS: u32 = 2;
    const DEFAULT_NUM_OUTPUTS: u32 = 2;

    const DEFAULT_AUX_INPUTS: Option<AuxiliaryIOConfig> = None;
    const DEFAULT_AUX_OUTPUTS: Option<AuxiliaryIOConfig> = None;

    const MIDI_INPUT: MidiConfig = MidiConfig::None;

    const SAMPLE_ACCURATE_AUTOMATION: bool = false;

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn accepts_bus_config(&self, config: &BusConfig) -> bool {
        config.num_input_channels == config.num_output_channels && config.num_input_channels > 0
    }

    // Initialize data here.

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _: &mut AuxiliaryBuffers,
        _: &mut impl ProcessContext,
    ) -> ProcessStatus {
        for channel_samples in buffer.iter_samples() {
            for sample in channel_samples {
                *sample *= -1.0;
            }
        }

        ProcessStatus::Normal
    }

    fn deactivate(&mut self) {}
}

impl ClapPlugin for PhaseInverter {
    const CLAP_ID: &'static str = "am.tonogr.phase-inverter";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Inverts the phase of the incoming signal.");
    const CLAP_MANUAL_URL: Option<&'static str> = None;
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Stereo,
        ClapFeature::Mono,
        ClapFeature::Utility,
    ];
}

nih_export_clap!(PhaseInverter);