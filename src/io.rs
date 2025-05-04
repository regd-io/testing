// Copyright 2025 Shingo OKAWA. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! This module contains a set of testing utilities of IO operations.

use std::io::BufRead;
use std::io::Write;
use std::{fs, io, path};

use tempfile::NamedTempFile;

/// Reads the contents of a file line by line using buffered I/O.
///
/// This function opens the file at the specified path and returns an iterator over its lines,
/// where each line is lazily read and returned as a `Result<String, io::Error>`. It utilizes
/// a `BufReader` for efficient I/O operations.
///
/// # Parameters
/// - `path`: The path to the file to be read. Accepts any type that implements `AsRef<Path>`.
///
/// # Returns
/// - An `Result` containing an iterator over the lines of the file, where each line
///   is represented as a `Result<String, io::Error>`.
///
/// # Examples
/// ```no_run
/// use regd_testing;
///
/// let lines = regd_testing::io::read_lines("Cargo.toml").expect("failed to open file");
/// for line in lines {
///     let line = line.expect("failed to read line");
///     println!("{}", line);
/// }
/// ```
pub fn read_lines(path: impl AsRef<path::Path>) -> io::Result<io::Lines<io::BufReader<fs::File>>> {
    let file = fs::File::open(path.as_ref())?;
    Ok(io::BufReader::new(file).lines())
}

/// Creates a new file at the specified path and writes the given content to it.
///
/// This function attempts to create a file at the provided path, writes the specified string
/// content to it, and flushes the buffer to ensure all data is persisted to disk.
///
/// # Parameters
/// - `path`: The target path for the new file. Accepts any type implementing `AsRef<Path>`.
/// - `content`: The string content to write into the newly created file. Accepts any type implementing `AsRef<str>`.
///
/// # Returns
/// - An `Result` containing the created `File` handle if successful, or an error if file creation or writing fails.
///
/// # Examples
/// ```no_run
/// use regd_testing;
///
/// let file = regd_testing::io::try_new_file("output.txt", "Hello, world!")
///     .expect("failed to create or write to file");
/// ```
pub fn try_new_file(
    path: impl AsRef<path::Path>,
    content: impl AsRef<str>,
) -> io::Result<fs::File> {
    let mut file = fs::File::create(path.as_ref())?;
    file.write_all(content.as_ref().as_bytes())?;
    file.sync_all()?;
    Ok(file)
}

/// Creates a new temporary file and writes the given content into it.
///
/// This function generates a temporary file using the system’s default temporary
/// directory, writes the provided string content into it, and returns a handle to the file.
/// The temporary file will be automatically deleted when dropped.
///
/// # Parameters
/// - `content`: The string content to write into the temporary file. Accepts any type implementing `AsRef<str>`.
///
/// # Returns
/// - An `Result` containing a `NamedTempFile` handle if the operation succeeds, or an error if it fails.
///
/// # Examples
/// ```no_run
/// use regd_testing;
///
/// let tempfile = regd_testing::io::try_new_tempfile("Temporary data")
///     .expect("failed to create temporary file");
///
/// println!("Temp file path: {:?}", tempfile.path());
/// ```
pub fn try_new_tempfile(content: impl AsRef<str>) -> io::Result<NamedTempFile> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "{}", content.as_ref())?;
    Ok(file)
}

/// Attempts to remove a file at the specified path, retrying up to 4 times on failure.
///
/// This function tries to delete the file located at the given path. If the removal
/// fails (e.g., due to temporary filesystem locks or race conditions), it will retry
/// up to four times before returning the final error.
///
/// # Parameters
/// - `path`: The path to the file to be removed. Accepts any type implementing `AsRef<Path>`.
///
/// # Returns
/// - An `Result` indicating success or failure. Returns `Ok(())` if the file is removed,
///   or an `Err` if all attempts fail.
///
/// # Examples
/// ```no_run
/// use regd_testing;
///
/// regd_testing::io::try_remove_file("temp.txt").expect("failed to remove file");
/// ```
pub fn try_remove_file(path: impl AsRef<path::Path>) -> io::Result<()> {
    for attempt in 1..=4 {
        match fs::remove_file(path.as_ref()) {
            Ok(_) => {
                return Ok(());
            }
            Err(e) => {
                if attempt < 4 {
                    continue;
                } else {
                    return Err(e);
                }
            }
        }
    }
    Ok(())
}
