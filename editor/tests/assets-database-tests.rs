use surreal_editor::AssetDatabase;

#[test]
fn database_should_build_manifest_from_assets() {
  // build the database
  let mut database = AssetDatabase::open("tests/assets", "tests/target").unwrap();

  // write database changes to disk
  database.flush_changes().unwrap();

  // check that the manifest was written to disk
  assert!(database.manifest_path().exists().unwrap());
}
