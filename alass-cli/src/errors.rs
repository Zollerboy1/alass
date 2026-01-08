// This file is part of the Rust library and binary `alass`.
//
// Copyright (C) 2017 kaegi
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use std::path::PathBuf;
use subparse::SubtitleFormat;

#[derive(Clone, Eq, PartialEq, Debug, thiserror::Error)]
pub enum InputFileError {
    #[error("processing video file '{0}' failed")]
    VideoFile(PathBuf),
    #[error("processing subtitle file '{0}' failed")]
    SubtitleFile(PathBuf),
}

#[derive(Clone, Eq, PartialEq, Debug, thiserror::Error)]
pub enum FileOperationError {
    #[error("failed to open file '{path}'")]
    FileOpen { path: PathBuf },
    #[error("failed to read file '{path}'")]
    FileRead { path: PathBuf },
    #[error("failed to write file '{path}'")]
    FileWrite { path: PathBuf },
}

#[derive(Clone, Eq, PartialEq, Debug, thiserror::Error)]
pub enum InputVideoError {
    #[error("failed to extract voice segments from file '{path}'")]
    FailedToDecode { path: PathBuf },
    #[error("failed to analyse audio segment for voice activity")]
    VadAnalysisFailed,
}

#[derive(Clone, Eq, PartialEq, Debug, thiserror::Error)]
pub enum InputSubtitleError {
    #[error("reading subtitle file '{0}' failed")]
    ReadingSubtitleFileFailed(PathBuf),
    #[error("unknown subtitle format for file '{0}'")]
    UnknownSubtitleFormat(PathBuf),
    #[error("parsing subtitle file '{0}' failed")]
    ParsingSubtitleFailed(PathBuf),
    #[error("retrieving subtitle file '{0}' failed")]
    RetrievingSubtitleLinesFailed(PathBuf),
}

#[derive(Clone, PartialEq, Debug, thiserror::Error)]
pub enum InputArgumentsError {
    #[error("expected value '{argument_name}' to be in range '{min}'-'{max}', found value '{value}'")]
    ValueNotInRange {
        argument_name: String,
        min: f64,
        max: f64,
        value: f64,
    },
    #[error("expected positive number for '{argument_name}', found '{value}'")]
    ExpectedPositiveNumber { argument_name: String, value: i64 },

    #[error("expected non-negative number for '{argument_name}', found '{value}'")]
    ExpectedNonNegativeNumber { argument_name: String, value: f64 },

    #[error("argument '{argument_name}' with value '{value}' could not be parsed")]
    ArgumentParseError { argument_name: String, value: String },
}

#[derive(Clone, Eq, PartialEq, Debug, thiserror::Error)]
pub enum TopLevelError {
    #[error(
        "output file '{input_file_path}' seems to have a different format than input file '{output_file_path}' with format '{}' (this program does not perform conversions)",
        input_file_format.get_name(),
    )]
    FileFormatMismatch {
        input_file_path: PathBuf,
        output_file_path: PathBuf,
        input_file_format: SubtitleFormat,
    },
    #[error("failed to change lines in the subtitle")]
    FailedToUpdateSubtitle,
    #[error("failed to generate data for subtitle")]
    FailedToGenerateSubtitleData,
    #[error("failed to instantiate subtitle file")]
    FailedToInstantiateSubtitleFile,
}
