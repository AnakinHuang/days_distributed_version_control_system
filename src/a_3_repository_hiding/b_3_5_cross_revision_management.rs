// days_dvcs/src/a_3_repository_hiding/b_3_5_cross_revision_management.rs
//

use super::b_3_1_repository_management::is_repository;
use super::b_3_2_revision_management::{
    commit, get_branch_or_revision_id, load_revision_metadata, RevisionMetadata,
};
use super::b_3_3_branch_management::{
    get_common_ancestor_and_count, load_branch_metadata, save_branch_metadata,
};

use crate::a_1_file_system_hiding::{
    b_1_1_file_interaction::{get_filename, get_parent, is_binary_file, read_file, write_file},
    b_1_2_directory_interaction::{
        check_directory, copy_directory, create_directory, delete_directory,
    },
    b_1_3_metadata_management::get_file_metadata,
    REMOTE,
};

use crate::a_1_file_system_hiding::b_1_2_directory_interaction::rename_directory;
use std::collections::HashSet;
use std::io;

const CONTEXT_LINES: usize = 3;

const EMPTY_FILE_HASH: &str = "00000000-0000-0000-0000-000000000000";

fn get_revisions(
    path: &str,
    branch_or_revision_id_1: &str,
    branch_or_revision_id_2: &str,
) -> Result<
    (
        String,
        String,
        String,
        RevisionMetadata,
        String,
        String,
        String,
        RevisionMetadata,
    ),
    io::Error,
> {
    let path_2 = &is_repository(path)?;
    let path_1 = if branch_or_revision_id_1 == REMOTE {
        &is_repository(&format!("{}/{}", path_2, REMOTE))?
    } else {
        path_2
    };
    let (_, branch_1, revision_id_1) = if branch_or_revision_id_1 == REMOTE {
        get_branch_or_revision_id(path_1, "")?
    } else {
        get_branch_or_revision_id(path_1, branch_or_revision_id_1)?
    };
    let (_, branch_2, revision_id_2) = get_branch_or_revision_id(path_2, branch_or_revision_id_2)?;

    let error = Err(io::Error::new(
        io::ErrorKind::InvalidInput,
        format!(
            "In repository '{}': No commits in branch '{}' yet...",
            get_filename(path_1),
            branch_1
        ),
    ));

    if revision_id_1.is_empty() {
        return error;
    }

    if revision_id_2.is_empty() {
        return error.map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "{}\nIn repository '{}': No commits in branch '{}' yet...",
                    e,
                    get_filename(path_2),
                    branch_2
                ),
            )
        });
    }

    let revision_metadata_1 = load_revision_metadata(path_1, &branch_1, &revision_id_1)?;
    let revision_metadata_2 = load_revision_metadata(path_2, &branch_2, &revision_id_2)?;

    Ok((
        path_1.to_string(),
        branch_1,
        revision_id_1,
        revision_metadata_1,
        path_2.to_string(),
        branch_2,
        revision_id_2,
        revision_metadata_2,
    ))
}

pub fn diff(
    path: &str,
    branch_or_revision_id_1: &str,
    branch_or_revision_id_2: &str,
) -> Result<String, io::Error> {
    let (
        path_1,
        branch_1,
        revision_id_1,
        revision_metadata_1,
        path_2,
        branch_2,
        revision_id_2,
        revision_metadata_2,
    ) = get_revisions(path, branch_or_revision_id_1, branch_or_revision_id_2)?;
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
                    diff_report.push_str(&format!(
                        "\x1b[33mFiles '{}/{}' and '{}/{}' are identical\x1b[0m\n",
                        &old_revision_metadata.id, file, &new_revision_metadata.id, file
                    ));
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
        || (file_old_hash == EMPTY_FILE_HASH && content_new != EMPTY_FILE_HASH)
        || (content_old != EMPTY_FILE_HASH && file_new_hash == EMPTY_FILE_HASH)
    {
        return Ok(format!(
            "\x1b[33mBinary files '{}/{}' and '{}/{}' differ\x1b[0m\n",
            revision_id_old, file, revision_id_new, file
        ));
    } else if content_old == content_new {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "Files '{}/{}' and '{}/{}' are identical in content but have different hash values '{}' : '{}'",
                revision_id_old, file, revision_id_new, file, file_old_hash, file_new_hash
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
        &format!(
            "{:o}",
            get_file_metadata(&format!("{}/{}", old_path, file))?.mode
        )
    };
    let mode_new = if new_path.is_empty() {
        "100644"
    } else {
        &format!(
            "{:o}",
            get_file_metadata(&format!("{}/{}", new_path, file))?.mode
        )
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

pub fn merge(
    path: &str,
    branch_or_revision_id_into: &str,
    branch_or_revision_id_from: &str,
    message: &str,
) -> Result<String, io::Error> {
    let (
        path_into,
        branch_into,
        revision_id_into,
        revision_metadata_into,
        path_from,
        branch_from,
        revision_id_from,
        revision_metadata_from,
    ) = get_revisions(path, branch_or_revision_id_into, branch_or_revision_id_from)?;

    if revision_id_into == revision_id_from {
        return Ok("No merge needed: Revisions are identical".to_string());
    }

    if revision_metadata_into.files.is_empty() && revision_metadata_from.files.is_empty() {
        return Ok("No files to merge yet...".to_string());
    }

    let mut branch_metadata = load_branch_metadata(&path_into, &branch_into)?;

    let merge_path = format!("{}/.dvcs/origin/{}/staging", path_into, branch_into);
    let stage_before_merge_path = format!(
        "{}/.dvcs/origin/{}/temp_staging_before_merge",
        path_into, branch_into
    );
    let content_into_path = format!(
        "{}/.dvcs/origin/{}/commits/{}",
        path_into, branch_into, revision_id_into
    );
    let content_from_path = format!(
        "{}/.dvcs/origin/{}/commits/{}",
        path_from, branch_from, revision_id_from
    );
    let mut merge_report = String::new();

    if check_directory(&stage_before_merge_path) {
        delete_directory(&stage_before_merge_path, true)?;
    }

    create_directory(&stage_before_merge_path)?;
    copy_directory(&merge_path, &stage_before_merge_path)?;

    let files: HashSet<&String> = revision_metadata_into
        .files
        .keys()
        .chain(revision_metadata_from.files.keys())
        .collect();
    let into = if branch_or_revision_id_into == REMOTE {
        "Remote HEAD"
    } else {
        branch_or_revision_id_into
    };
    let from = if branch_or_revision_id_from.is_empty() {
        "Local HEAD"
    } else {
        branch_or_revision_id_from
    };

    for file in files.clone() {
        let hash_into = revision_metadata_into.files.get(file);
        let hash_from = revision_metadata_from.files.get(file);
        let content_into = read_file(&format!("{}/{}", content_into_path, file)).ok();
        let content_from = read_file(&format!("{}/{}", content_from_path, file)).ok();
        let staging_dir = format!("{}/{}", merge_path, get_parent(file));
        let staging_path = format!("{}/{}", merge_path, file);
        let (_, revision_id) = get_common_ancestor_and_count(
            &path_into,
            &branch_into,
            &revision_id_into,
            &path_from,
            &branch_from,
            &revision_id_from,
        )?;
        let ancestor_content = if let Some(revision_id_ancestor) = revision_id {
            read_file(&format!(
                "{}/.dvcs/origin/{}/commits/{}/{}",
                path_into, branch_into, revision_id_ancestor, file
            ))
            .ok()
        } else {
            None
        };

        match (content_into, content_from) {
            (Some(content_into), Some(content_from)) => {
                if content_into == content_from {
                    match (hash_into, hash_from) {
                        (Some(into), Some(from)) => {
                            if into == from {
                                if !check_directory(&staging_dir) {
                                    create_directory(&staging_dir)?;
                                }
                                write_file(&staging_path, &content_into)?;
                                merge_report.push_str(&format!("File unchanged: '{}'\n", file));
                            } else {
                                return Err(io::Error::new(
                                    io::ErrorKind::InvalidData,
                                    format!(
                                        "Files '{}/{}' and '{}/{}' are identical in content but have different hash values '{}' : '{}'",
                                        revision_id_into, file, revision_id_from, file, into, from
                                    ),
                                ));
                            }
                        }
                        (Some(_), None) | (None, Some(_)) => {
                            return Err(io::Error::new(
                                io::ErrorKind::InvalidData,
                                format!(
                                    "Files '{}/{}' and '{}/{}' are identical in content but hash value of one file is missing",
                                    revision_id_into, file, revision_id_from, file
                                ),
                            ));
                        }
                        _ => unreachable!(),
                    }
                } else if let Some(ancestor) = ancestor_content {
                    let merged_content =
                        merge_contents(&ancestor, into, &content_into, from, &content_from);
                    if !check_directory(&staging_dir) {
                        create_directory(&staging_dir)?;
                    }
                    write_file(&staging_path, &merged_content)?;
                    merge_report.push_str(&format!("File merged: '{}'\n", file));
                } else {
                    let merged_content = merge_conflict(into, &content_into, from, &content_from);
                    if !check_directory(&staging_dir) {
                        create_directory(&staging_dir)?;
                    }
                    write_file(&staging_path, &merged_content)?;
                    merge_report.push_str(&format!(
                        "Conflict in file (no common ancestor): '{}'\n",
                        file
                    ));
                }
            }
            (Some(content_into), None) => {
                if !check_directory(&staging_dir) {
                    create_directory(&staging_dir)?;
                }
                write_file(&staging_path, &content_into)?;
                merge_report.push_str(&format!(
                    "File removed in revision '{}': '{}'\n",
                    revision_id_from, file
                ));
            }
            (None, Some(content_from)) => {
                if !check_directory(&staging_dir) {
                    create_directory(&staging_dir)?;
                }
                write_file(&staging_path, &content_from)?;
                merge_report.push_str(&format!(
                    "File added in revision '{}': '{}'\n",
                    revision_id_from, file
                ));
            }
            _ => unreachable!(),
        }
    }

    let new_staging: HashSet<String> = branch_metadata
        .staging
        .iter()
        .chain(files)
        .cloned()
        .collect();

    branch_metadata.staging = Vec::from_iter(new_staging);
    save_branch_metadata(&path_into, &branch_into, &branch_metadata)?;

    if let Err(e) = commit(&path_into, message) {
        delete_directory(&merge_path, true)?;
        rename_directory(&stage_before_merge_path, &merge_path)?;
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "Merge failed during commit. Reverting to previous staging area: {}", e
            ),
        ));
    }

    delete_directory(&stage_before_merge_path, true)?;

    Ok(merge_report)
}

fn merge_contents(
    ancestor: &str,
    into: &str,
    content_into: &str,
    from: &str,
    content_from: &str,
) -> String {
    let ancestor_lines: Vec<&str> = ancestor.lines().collect();
    let content_into_lines: Vec<&str> = content_into.lines().collect();
    let content_from_lines: Vec<&str> = content_from.lines().collect();
    let mut conflicted_into_lines: Vec<String> = Vec::new();
    let mut conflicted_from_lines: Vec<String> = Vec::new();
    let mut agreed_lines: Vec<String> = Vec::new();

    let mut merged_lines: Vec<String> = Vec::new();
    let max_lines = ancestor_lines
        .len()
        .max(content_into_lines.len())
        .max(content_from_lines.len());

    for i in 0..max_lines {
        let ancestor_line = ancestor_lines.get(i).unwrap_or(&"").to_string();
        let content_into_line = content_into_lines.get(i).unwrap_or(&"").to_string();
        let content_from_line = content_from_lines.get(i).unwrap_or(&"").to_string();
        let mut new_line = &ancestor_line;
        let mut agreed = false;

        if content_from_line == content_into_line {
            new_line = &content_from_line;
            agreed = true;
        } else if content_into_line == ancestor_line || content_from_line == ancestor_line {
            agreed = true;
        } else {
            conflicted_from_lines.push(content_from_line);
            conflicted_into_lines.push(content_into_line);
        }

        if agreed {
            if !conflicted_from_lines.is_empty() || !conflicted_into_lines.is_empty() {
                merged_lines.extend(agreed_lines.drain(..));
                merged_lines.push(format_conflict(
                    into,
                    conflicted_into_lines.drain(..).collect(),
                    from,
                    conflicted_from_lines.drain(..).collect(),
                ));
            }
            agreed_lines.push(new_line.to_string());
        }
    }

    merged_lines.extend(agreed_lines.drain(..));

    if !conflicted_from_lines.is_empty() || !conflicted_into_lines.is_empty() {
        merged_lines.push(format_conflict(
            into,
            conflicted_into_lines.drain(..).collect(),
            from,
            conflicted_from_lines.drain(..).collect(),
        ));
    }

    merged_lines.join("\n")
}

fn merge_conflict(into: &str, content_into: &str, from: &str, content_from: &str) -> String {
    let mut merged_lines = Vec::new();
    let mut conflicted_into_lines: Vec<String> = Vec::new();
    let mut conflicted_from_lines: Vec<String> = Vec::new();
    let mut unchanged_lines: Vec<String> = Vec::new();

    for diff in diff::lines(content_from, content_into) {
        match diff {
            diff::Result::Left(line) => {
                conflicted_from_lines.push(line.to_string());
            }
            diff::Result::Right(line) => {
                conflicted_into_lines.push(line.to_string());
            }
            diff::Result::Both(line, _) => {
                if !conflicted_from_lines.is_empty() || !conflicted_into_lines.is_empty() {
                    merged_lines.extend(unchanged_lines.drain(..));
                    merged_lines.push(format_conflict(
                        into,
                        conflicted_into_lines.drain(..).collect(),
                        from,
                        conflicted_from_lines.drain(..).collect(),
                    ));
                }
                unchanged_lines.push(line.to_string());
            }
        }
    }

    merged_lines.extend(unchanged_lines.drain(..));

    if !conflicted_from_lines.is_empty() || !conflicted_into_lines.is_empty() {
        merged_lines.push(format_conflict(
            into,
            conflicted_into_lines.drain(..).collect(),
            from,
            conflicted_from_lines.drain(..).collect(),
        ));
    }

    merged_lines.join("\n")
}

fn format_conflict(
    into: &str,
    conflicted_into_lines: Vec<String>,
    from: &str,
    conflicted_from_lines: Vec<String>,
) -> String {
    format!(
        "<<<<<<< {}\n{}\n=======\n{}\n>>>>>>> {}",
        into,
        conflicted_from_lines.join("\n"),
        conflicted_into_lines.join("\n"),
        from
    )
}
