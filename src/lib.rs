use self::{error::prelude::*, flashcard::Flashcard};
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, path::Path};
use uuid::Uuid;

/// Deck is a storage of flash cards and files linked to them.
#[derive(Serialize, Deserialize, Debug)]
pub struct Deck {
	/// Unique deck identifier.
	id: String,

	/// Non-unique convenient deck name.
	name: String,

	/// Flash cards stored in this deck
	cards: Vec<Flashcard>,

	/// Storage of files linked with flash cards.
	storage: RefCell<Vec<FileDesc>>,
}

impl Deck {
	/// Deck file extension.
	pub(crate) const DECK_FILE_EXT: &'static str = ".deck";

	/// How to name storage directory inside zipped deck file.
	const DECK_FILES_STORAGE_PATH: &'static str = "storage";

	/// How to name raw binary deck file inside zipped deck file.
	const DECK_FILES_DECK_PATH: &'static str = "deck";

	/// Creates a new [`Deck`].
	pub fn new(name: impl Into<String>) -> Self {
		Self {
			id: Uuid::new_v4().to_string(),
			name: name.into(),
			cards: Vec::new(),
			storage: RefCell::new(Vec::new()),
		}
	}

	/// Serializes deck into binary file, puts all linked with flash cards files
	/// in one directory and archives all these files in .tar.gz
	/// format. Resulting file has [`Self::DECK_FILE_EXT`] extension.
	pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
		use flate2::write::GzEncoder;
		use std::fs::{self, File};
		use tempfile::tempdir;

		error_kind!(SavingDeck);

		let root_dir = tempdir().map_err(error::err!())?;
		let working_dir = root_dir.path().join("deck_files");
		let storage_dir_path = working_dir.join(Self::DECK_FILES_STORAGE_PATH);
		let deck_path = working_dir.join(Self::DECK_FILES_DECK_PATH);

		fs::create_dir_all(&storage_dir_path).map_err(err!())?;

		for fd in self.storage.borrow().iter() {
			fd.save(&storage_dir_path)?;
		}

		let deck_file = File::create(&deck_path).map_err(err!())?;

		bincode::serialize_into(&deck_file, self).map_err(err!())?;

		let archive_path = root_dir.path().join("deck.tar.gz");
		let archive = File::create(&archive_path).map_err(err!())?;
		let mut tar =
			tar::Builder::new(GzEncoder::new(archive, Default::default()));

		tar.append_dir_all(".", &working_dir).map_err(err!())?;
		let _ = tar.into_inner().map_err(err!())?;

		let output_file_name = format!(
			"{name}{ext}",
			name = self.name.replace(' ', "_"),
			ext = Self::DECK_FILE_EXT
		);

		fs::copy(archive_path, path.as_ref().join(output_file_name))
			.map_err(err!())?;

		Ok(())
	}

	/// Deserializes a new [`Deck`] instance from deck file with `path`
	/// path. `storage_path` is path to directory to save files linked with
	/// flash cards (storage).
	pub fn from_file<D, S>(path: D, storage_path: S) -> Result<Self>
	where
		D: AsRef<Path>,
		S: AsRef<Path>,
	{
		use flate2::read::GzDecoder;
		use std::fs::File;
		use tempfile::tempdir;

		error_kind!(GettingDeckFromFile);

		let dir = tempdir().map_err(err!())?;
		let archive_file = File::open(path).map_err(err!())?;
		let mut archive = tar::Archive::new(GzDecoder::new(archive_file));

		archive.unpack(dir.path()).map_err(err!())?;

		fs_extra::copy_items(
			&[dir.path().join(Self::DECK_FILES_STORAGE_PATH)],
			storage_path,
			&Default::default(),
		)
		.map_err(err!())?;

		let deck_file = File::open(dir.path().join(Self::DECK_FILES_DECK_PATH))
			.map_err(err!())?;

		let deck: Self =
			bincode::deserialize_from(deck_file).map_err(err!())?;

		Ok(deck)
	}

	/// Close all opened program file descriptors.
	fn close_fds(&self) {
		for fd in self.storage.borrow_mut().iter_mut() {
			fd.close();
		}
	}

	fn id(&self) -> &str {
		&self.id
	}

	fn name(&self) -> &str {
		&self.name
	}
}

/// `FileDesc` is a program file descriptor. It's used to link files with flash
/// cards and work with them dynamically. [`Vec<FileDesc>`] is called
/// `storage`. In file system, `storage` is a directory with uniquely-named
/// files, in other words, saved data provided by program file descriptors.
#[derive(Serialize, Deserialize, Debug)]
pub struct FileDesc {
	/// Unique file descriptor identifier.
	id: String,

	/// File extension without dot.
	ext: String,

	/// How many flash cards reference to this file descriptor.
	rc: u32,

	/// File data stored in this program file descriptor.
	#[serde(skip)]
	data: Option<Vec<u8>>,
}

impl FileDesc {
	/// Create a new program file descriptor. `path` is path to file on the file
	/// system to open. `rc` is how many flash cards reference to this program
	/// file descriptor.
	fn new(path: impl AsRef<Path>, rc: u32) -> Result<Self> {
		use std::fs;
		let path = path.as_ref();
		Ok(Self {
			id: Uuid::new_v4().to_string(),
			ext: path
				.extension()
				.and_then(|ext| ext.to_str())
				.map(|ext| ext.to_string())
				.unwrap_or_default(),
			data: Some(fs::read(path).map_err(err!(CreatingFileDesc))?),
			rc,
		})
	}

	/// Write data of the file located in a storage with provided path to this
	/// file descriptor.
	fn open(&mut self, storage_path: impl AsRef<Path>) -> Result<()> {
		use std::fs;
		self.data = Some(
			fs::read(
				storage_path
					.as_ref()
					.join(&self.id)
					.with_extension(&self.ext),
			)
			.map_err(err!(OpeningFileDesc))?,
		);
		Ok(())
	}

	/// Remove file data stored in this program file descriptor.
	fn close(&mut self) {
		self.data = None;
	}

	/// Save data stored in this program file descriptor to unique storage file.
	fn save(&self, storage_path: impl AsRef<Path>) -> Result<()> {
		use std::fs::File;
		use std::io::Write;

		error_kind!(SavingFileDesc);

		if self.data.is_none() {
			return Ok(());
		}

		let data = self.data.as_ref().unwrap();
		let path = storage_path
			.as_ref()
			.join(&self.id)
			.with_extension(&self.ext);
		let mut file = File::create(path).map_err(err!())?;

		file.write_all(data).map_err(err!())?;

		Ok(())
	}

	/// Check if there's some data stored by this program file descriptor.
	fn is_opened(&self) -> bool {
		self.data.is_some()
	}
}

/// Flash card realted abstractions.
pub mod flashcard {
	use serde::{Deserialize, Serialize};

	/// Flash card is a small container of information which should be memorized.
	#[derive(Serialize, Deserialize, Debug)]
	pub struct Flashcard {
		fields: Vec<Field>,
		sides: Vec<Side>,
		auto_rendering: bool,
	}

	/// Data which should be showed on flash card's sides is defined in fields.
	#[derive(Serialize, Deserialize, Debug)]
	pub struct Field {
		data: String,
	}

	/// All flash card's data is represented on its sides.
	#[derive(Serialize, Deserialize, Debug)]
	pub struct Side {
		data: String,
	}
}

/// Module which's used by entire crate to handle errors.
pub(crate) mod error {
	use std::{error, fmt};

	/// Convenient module to bring everything that crate functions may use to
	/// handle errors.
	pub(crate) mod prelude {
		pub(crate) use super::Result;
		pub(crate) use super::{err, error_kind};
	}

	/// Convenient version of `Result` used by all functions in this crate.
	pub(crate) type Result<T> = std::result::Result<T, Error>;

	/// Error returned by most of functions in this crate.
	#[derive(Debug)]
	pub struct Error {
		error: Box<dyn error::Error + Send + Sync>,
		kind: Kind,
		file: &'static str,
		line: u32,
		column: u32,
	}

	impl Error {
		pub(crate) fn new<E>(
			error: E,
			kind: Kind,
			file: &'static str,
			line: u32,
			column: u32,
		) -> Self
		where
			E: Into<Box<dyn error::Error + Send + Sync>>,
		{
			Self {
				error: error.into(),
				kind,
				file,
				line,
				column,
			}
		}
	}

	impl fmt::Display for Error {
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			if cfg!(debug_assertions) {
				write!(
					f,
					"{kind} in {file}:{line}:{column}: {error}",
					kind = self.kind,
					error = self.error,
					file = self.file,
					line = self.line,
					column = self.column
				)
			} else {
				write!(
					f,
					"{kind}: {error}",
					kind = self.kind,
					error = self.error
				)
			}
		}
	}

	impl error::Error for Error {}

	/// Kind of errors returned by some functions in this crate.
	// We're allowing dead code here because some variants don't have to be
	// constructed directly, but instead with self::error::err! macro.
	#[allow(dead_code)]
	#[derive(Debug, Clone, Copy)]
	pub(crate) enum Kind {
		SavingDeck,
		GettingDeckFromFile,
		SavingFileDesc,
		CreatingFileDesc,
		OpeningFileDesc,
	}

	impl fmt::Display for Kind {
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			use crate::Deck;
			use Kind::*;
			write!(
				f,
				"Error while {}",
				match self {
					SavingDeck => format!(
						"saving deck to .{ext} file",
						ext = Deck::DECK_FILE_EXT
					),
					GettingDeckFromFile => format!(
						"getting deck from .{ext} file",
						ext = Deck::DECK_FILE_EXT
					),
					SavingFileDesc => "saving program file descriptor".into(),
					CreatingFileDesc =>
						"creating program file descriptor".into(),
					OpeningFileDesc => "opening program file descriptor".into(),
				}
			)
		}
	}

	/// Initialize kind of errors of a function. Should be used before [`err!`]
	/// macro to allow using it without arguments.
	macro_rules! error_kind {
		($kind:ident) => {
			const _ERROR_KIND: $crate::error::Kind = $crate::error::Kind::$kind;
		};
	}

	/// Return type which can be converted into [`Error`]. Note that
	/// [`error_kind`] macro should be called before this function to initialize
	/// kind of errors which can be handled by that function, if there're no
	/// arguments provided to this macro. Otherwise, use [`Kind`]::$kind
	/// [`ErrorKind`](Kind), where $kind is a first argument.
	macro_rules! err {
		() => {
			|error| {
				$crate::error::Error::new(
					error,
					_ERROR_KIND,
					file!(),
					line!(),
					column!(),
				)
			}
		};
		($kind:ident) => {
			|error| {
				$crate::error::Error::new(
					error,
					$crate::error::Kind::$kind,
					file!(),
					line!(),
					column!(),
				)
			}
		};
	}

	pub(crate) use err;
	pub(crate) use error_kind;
}
