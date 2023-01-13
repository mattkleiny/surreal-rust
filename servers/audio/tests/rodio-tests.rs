use surreal_audio::AudioServer;

#[test]
fn test_rodio_backend() {
  let _server = AudioServer::from_rodio().unwrap();
}
