use crate::AudioBackend;

/// A [`AudioBackend`] based on Rodio.
pub struct RodioBackend {
  _output_stream: rodio::OutputStream,
  _output_stream_handle: rodio::OutputStreamHandle,
}

impl RodioBackend {
  /// Creates a new [`RodioBackend`].
  pub fn new() -> surreal::Result<Self> {
    let (output_stream, output_stream_handle) = rodio::OutputStream::try_default()?;

    Ok(Self {
      _output_stream: output_stream,
      _output_stream_handle: output_stream_handle,
    })
  }
}

impl AudioBackend for RodioBackend {}
