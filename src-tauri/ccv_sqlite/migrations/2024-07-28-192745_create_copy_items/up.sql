-- Your SQL goes here

CREATE TABLE copy_items (
  id            INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  created       TIMESTAMP NOT NULL,
  last_reuse    TIMESTAMP NOT NULL,
  category      VARCHAR(10) NOT NULL CHECK( category IN ('Files','Html','Image','Rtf','Text','Unknown') ),
  text          TEXT,
  html          TEXT,
  rtf           TEXT,
  files_text    TEXT,
  image         TEXT
);

CREATE TABLE file_infos (
  id                INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  copy_item_id      INTEGER NOT NULL,
  full_path         TEXT NOT NULL,
  file_name         VARCHAR(256),
  directory_path    TEXT,
  icon_base64       TEXT,
  is_directory      BOOLEAN NOT NULL,
  FOREIGN KEY (copy_item_id) REFERENCES copy_items(id) ON DELETE CASCADE ON UPDATE NO ACTION
);