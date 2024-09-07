use log::LevelFilter;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn try_init() {
    let mut builder = pretty_env_logger::formatted_builder();

    let _ = builder
        .filter(None, LevelFilter::Trace)
        .format_timestamp_millis()
        //.write_style(WriteStyle::Always)
        .try_init();
}
