// days_dvcs/src/a_3_repository_hiding/b_3_5_cross_revision_management.rs
//

use super::b_3_1_repository_management::is_repository;
use super::b_3_2_revision_management::{get_branch_or_revision_id, load_revision_metadata};

use crate::a_1_file_system_hiding::{
    b_1_1_file_interaction::{is_binary_file, read_file},
    b_1_3_metadata_management::get_mode,
    REMOTE,
};

use std::collections::HashSet;
use std::io;
const CONTEXT_LINES: usize = 3;

const EMPTY_FILE_HASH: &str = "00000000-0000-0000-0000-000000000000";

pub fn diff(
    path: &str,
    branch_or_revision_id_1: &str,
    branch_or_revision_id_2: &str,
) -> Result<String, io::Error> {
    let path_2 = &is_repository(path)?;
    let path_1 = if branch_or_revision_id_1 == REMOTE {
        &is_repository(&format!("{}/{}", path_2, REMOTE))?
    } else {
        path_2
    };
    let path_2 = &is_repository(path)?;
    let (_, branch_1, revision_id_1) = if branch_or_revision_id_1 == REMOTE {
        get_branch_or_revision_id(path_1, "")?
    } else {
        get_branch_or_revision_id(path_1, branch_or_revision_id_1)?
    };
    let (_, branch_2, revision_id_2) = get_branch_or_revision_id(path_2, branch_or_revision_id_2)?;
    let revision_metadata_1 = load_revision_metadata(path_1, &branch_1, &revision_id_1)?;
    let revision_metadata_2 = load_revision_metadata(path_2, &branch_2, &revision_id_2)?;
    let old_path = if revision_metadata_1.timestamp < revision_metadata_2.timestamp {
        &format!(
            "{}/.dvcs/origin/{}/commits/{}",
            path_1, branch_1, &revision_id_1
        )
    } else {
        &format!(
            "{}/.dvcs/origin/{}/commits/{}",
            path_2, branch_2, &revision_id_2
        )
    };
    let new_path = if revision_metadata_1.timestamp < revision_metadata_2.timestamp {
        &format!(
            "{}/.dvcs/origin/{}/commits/{}",
            path_2, branch_2, &revision_id_2
        )
    } else {
        &format!(
            "{}/.dvcs/origin/{}/commits/{}",
            path_1, branch_1, &revision_id_1
        )
    };
    let old_revision_metadata = if revision_metadata_1.timestamp < revision_metadata_2.timestamp {
        revision_metadata_1.clone()
    } else {
        revision_metadata_2.clone()
    };
    let new_revision_metadata = if revision_metadata_1.timestamp < revision_metadata_2.timestamp {
        revision_metadata_2
    } else {
        revision_metadata_1
    };
    let mut diff_report = String::new();
    let files: HashSet<&String> = old_revision_metadata
        .files
        .keys()
        .chain(new_revision_metadata.files.keys())
        .collect();

    for file in files {
        let old_hash = old_revision_metadata.files.get(file);
        let new_hash = new_revision_metadata.files.get(file);

        match (old_hash, new_hash) {
            (Some(old), Some(new)) => {
                if old != new {
                    // File exists in both revisions but has changed
                    diff_report.push_str(&diff_files(
                        file,
                        &old_revision_metadata.id,
                        old_path,
                        old,
                        &new_revision_metadata.id,
                        new_path,
                        new,
                    )?)
                } else {
                    // File exists in both revisions and is identical
                    diff_report.push_str(
                        format!(
                            "\x1b[32mFiles {}/{} and {}/{} are identical\x1b[0m\n",
                            &old_revision_metadata.id, file, &new_revision_metadata.id, file
                        )
                        .as_str(),
                    );
                }
            }
            (Some(old), None) => {
                // File exists only in the old revision (deleted)
                diff_report.push_str(&diff_files(
                    file,
                    &old_revision_metadata.id,
                    old_path,
                    old,
                    &new_revision_metadata.id,
                    "",
                    EMPTY_FILE_HASH,
                )?)
            }
            (None, Some(new)) => {
                // File exists only in the new revision (added)
                diff_report.push_str(&diff_files(
                    file,
                    &old_revision_metadata.id,
                    "",
                    EMPTY_FILE_HASH,
                    &new_revision_metadata.id,
                    new_path,
                    new,
                )?)
            }
            _ => unreachable!(), // This should never happen due to the union of files
        }
    }

    Ok(diff_report)
}

fn diff_files(
    file: &str,
    revision_id_old: &str,
    old_path: &str,
    file_old_hash: &str,
    revision_id_new: &str,
    new_path: &str,
    file_new_hash: &str,
) -> Result<String, io::Error> {
    let content_old = if old_path.is_empty() {
        String::new()
    } else {
        read_file(&format!("{}/{}", old_path, file))?
    };

    let content_new = if new_path.is_empty() {
        String::new()
    } else {
        read_file(&format!("{}/{}", new_path, file))?
    };

    if (is_binary_file(&content_old) || is_binary_file(&content_new))
        || (content_old.is_empty() && !content_new.is_empty())
        || (!content_old.is_empty() && content_new.is_empty())
    {
        return Ok(format!(
            "\x1b[32mBinary files {}/{} and {}/{} differ\x1b[0m\n",
            revision_id_old, file, revision_id_new, file
        ));
    } else if content_old == content_new {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "Files {}/{} and {}/{} are identical in content but have different hash value",
                revision_id_old, file, revision_id_new, file
            ),
        ));
    }

    let header = diff_files_header(
        file,
        revision_id_old,
        old_path,
        file_old_hash,
        revision_id_new,
        new_path,
        file_new_hash,
    )?;
    let body = diff_files_body(&content_old, &content_new)?;

    Ok(format!("{}{}\n", header, body))
}

fn diff_files_header(
    file: &str,
    revision_id_old: &str,
    old_path: &str,
    file_old_hash: &str,
    revision_id_new: &str,
    new_path: &str,
    file_new_hash: &str,
) -> Result<String, io::Error> {
    let mode_old = if old_path.is_empty() {
        "100644"
    } else {
        &format!("{:o}", get_mode(&format!("{}/{}", old_path, file))?)
    };
    let mode_new = if new_path.is_empty() {
        "100644"
    } else {
        &format!("{:o}", get_mode(&format!("{}/{}", new_path, file))?)
    };
    let header_head = format!(
        "diff --dvcs {}/{} {}/{}\nindex {}..{}",
        revision_id_old, file, revision_id_new, file, file_old_hash, file_new_hash,
    );
    let header_mode = if old_path.is_empty() {
        format!(" {}\n", mode_new)
    } else if new_path.is_empty() || mode_old == mode_new {
        format!(" {}\n", mode_old)
    } else {
        format!("\nold mode {}\nnew mode {}\n", mode_old, mode_new)
    };
    let file_old = if old_path.is_empty() {
        "/dev/null".to_string()
    } else {
        format!("{}/{}", revision_id_old, file)
    };
    let file_new = if new_path.is_empty() {
        "/dev/null".to_string()
    } else {
        format!("{}/{}", revision_id_new, file)
    };

    Ok(format!(
        "{}{}--- {}\n+++ {}\n",
        header_head, header_mode, file_old, file_new,
    ))
}

fn diff_files_body(content_old: &str, content_new: &str) -> Result<String, io::Error> {
    let mut all_lines = Vec::new();

    for diff in diff::lines(content_old, content_new) {
        match diff {
            diff::Result::Left(line) => all_lines.push(format!("\x1b[38;5;214m-{}\x1b[0m", line)), // Removed line
            diff::Result::Right(line) => all_lines.push(format!("\x1b[32m+{}\x1b[0m", line)), // Added line
            diff::Result::Both(line, _) => all_lines.push(format!("\x1b[0m {}\x1b[0m", line)), // Unchanged line
        }
    }

    match (content_old.ends_with('\n'), content_new.ends_with('\n')) {
        (true, true) => (),
        (true, false) => all_lines.push("\x1b[32m+\\ No newline at end of file\x1b[0m".to_string()),
        (false, true) => {
            all_lines.push("\x1b[38;5;214m-\\ No newline at end of file\x1b[0m".to_string())
        }
        (false, false) => all_lines.push("\x1b[0m\\ No newline at end of file\x1b[0m".to_string()),
    }

    let mut diff_chunks = Vec::new();
    let mut current_chunk = Vec::new();
    let mut last_change_index: Option<usize> = None;

    for (i, line) in all_lines.iter().enumerate() {
        if line.starts_with("\x1b[38;5;214m-") || line.starts_with("\x1b[32m+") {
            if let Some(last_index) = last_change_index {
                if i > last_index + CONTEXT_LINES {
                    trim_context(&mut current_chunk);
                    diff_chunks.push(format_chunk(&current_chunk));
                    current_chunk.clear();
                }
            }

            last_change_index = Some(i);
        }

        current_chunk.push((i, line.clone()));
    }

    if !current_chunk.is_empty() {
        trim_context(&mut current_chunk);
        diff_chunks.push(format_chunk(&current_chunk));
    }

    Ok(diff_chunks.join("\n"))
}

fn trim_context(chunk: &mut Vec<(usize, String)>) {
    if let (Some(start), Some(end)) = (
        chunk.iter().position(|(_, line)| {
            line.starts_with("\x1b[38;5;214m-") || line.starts_with("\x1b[32m+")
        }),
        chunk.iter().rposition(|(_, line)| {
            line.starts_with("\x1b[38;5;214m-") || line.starts_with("\x1b[32m+")
        }),
    ) {
        let start = start.saturating_sub(CONTEXT_LINES);
        let end = (end + CONTEXT_LINES + 1).min(chunk.len());
        *chunk = chunk[start..end].to_vec();
    }
}

fn format_chunk(chunk: &Vec<(usize, String)>) -> String {
    let start_old = match chunk
        .iter()
        .find(|(_, line)| !line.starts_with("\x1b[38;5;214m-"))
    {
        Some((index, _)) => index + 1,
        None => 0,
    };
    let start_new = match chunk
        .iter()
        .find(|(_, line)| !line.starts_with("\x1b[32m+"))
    {
        Some((index, _)) => index + 1,
        None => 0,
    };

    let count_old = chunk
        .iter()
        .filter(|(_, line)| !line.starts_with("\x1b[38;5;214m-"))
        .count();
    let count_new = chunk
        .iter()
        .filter(|(_, line)| !line.starts_with("\x1b[32m+"))
        .count();
    let header = format!(
        "\x1b[1;36m@@ -{},{} +{},{} @@\x1b[0m\n",
        start_old, count_old, start_new, count_new
    );
    let body = chunk
        .iter()
        .map(|(_, line)| line.clone())
        .collect::<Vec<String>>()
        .join("\n");

    format!("{}{}", header, body)
}
