use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

/* Create an archive from multiple files
   +----------------+
   | file count     | 8 bytes
   +----------------+
   | filename size  |
   | filename       |
   | file size      |
   | file bytes     |
   +----------------+
   | filename size  |
   | filename       |
   | file size      |
   | file bytes     |
   +----------------+ */
pub fn create_archive(paths: &[PathBuf]) -> io::Result<Vec<u8>> {

    let mut archive = Vec::new();

    // reserve space for file count.
    archive.extend(0u64.to_le_bytes());

    let mut file_count = 0u64;

    for path in paths {

        if path.is_file() {

            add_file(
                &mut archive,
                path,
                &mut file_count
            )?;

        } else if path.is_dir() {

            add_directory(
                &mut archive,
                path,
                &mut file_count
            )?;
        }
    }

    // overwrite first 8 bytes
    archive[0..8]
        .copy_from_slice(
            &file_count.to_le_bytes()
        );

    Ok(archive)
}


/// Add a single file into archive
fn add_file(
    archive: &mut Vec<u8>,
    path: &Path,
    file_count: &mut u64,
) -> io::Result<()> {

    let mut file = File::open(path)?;

    *file_count += 1;

    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    let filename = path
        .file_name()
        .ok_or(
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "File has no name"
            )
        )?
        .to_string_lossy()
        .to_string();

    // Store filename length.
    let name_len = filename.len() as u64;
    archive.extend(name_len.to_le_bytes());

    // Store filename.
    archive.extend(filename.as_bytes());

    // Store file size.
    let data_len = data.len() as u64;
    archive.extend(data_len.to_le_bytes());

    // Store file contents/bytes.
    archive.extend(data);

    Ok(())
}


/// Recursively add each files within a folder.
fn add_directory(
    archive: &mut Vec<u8>,
    directory: &Path,
    file_count: &mut u64,
) -> io::Result<()> {

    // A folder can contain files or another folder, and folders
    // are recurively called to add all file contents.
    for entry in fs::read_dir(directory)? {

        let entry = entry?;
        let path = entry.path();

        if path.is_file() {

            add_file(archive, &path, file_count)?;

        } else if path.is_dir() {

            add_directory(archive, &path, file_count)?;

        }
    }

    Ok(())
}


/// Extract archive contents
pub fn extract_archive(
    archive: &[u8],
    output_directory: &Path
) -> io::Result<()> {

    fs::create_dir_all(output_directory)?;

    // Keeps track of the space of each file metadata.
    let mut cursor = 0;

    // Read number of files
    let file_count = read_u64(archive, &mut cursor)?;

    for _ in 0..file_count {

        // filename length
        let name_len = read_u64(archive, &mut cursor)? as usize;

        // filename
        let filename_bytes =
            archive.get(cursor..cursor + name_len)
            .ok_or(
                io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "Invalid filename data"
                )
            )?;

        let filename =
            String::from_utf8(
                filename_bytes.to_vec()
            )
            .map_err(|_| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid filename encoding"
                )
            })?;

        cursor += name_len;

        // file size
        let file_size = read_u64(archive, &mut cursor)? as usize;

        // file data
        let data =
            archive.get(cursor..cursor + file_size)
            .ok_or(
                io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "Invalid archive data"
                )
            )?;

        cursor += file_size;

        // recreate path
        let original_path = Path::new(&filename);

        let mut output_path =
            output_directory.join(
                original_path.file_name()
                .unwrap()
            );


        let mut counter = 1;

        // Restore all files encryted and ensuring they titled with their
        // original extension.
        while output_path.exists() {

            let extension =
                original_path
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .unwrap_or("");


            output_path =
                output_directory.join(
                    format!(
                        "{}_{}.{}",
                        original_path.file_stem()
                            .unwrap()
                            .to_string_lossy(),
                        counter,
                        extension
                    )
                );

            counter += 1;
        }

        // Write byte data back into file format to restore the data.
        let mut file = File::create(output_path)?;

        file.write_all(data)?;

    }

    Ok(())
}



fn read_u64(
    data: &[u8],
    cursor: &mut usize
) -> io::Result<u64> {

    let bytes =
        data.get(*cursor..*cursor + 8)
        .ok_or(
            io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Invalid archive"
            )
        )?;

    *cursor += 8;

    Ok(
        u64::from_le_bytes(
            bytes.try_into().unwrap()
        )
    )
}