// days_dvcs/src/a_3_repository_hiding/b_3_4_synchronization_handler.rs
//

use super::b_3_1_repository_management::{is_repository, load_repo_metadata, save_repo_metadata};
use super::b_3_2_revision_management::load_revision_metadata;
use super::b_3_3_branch_management::{init_branch, load_branch_metadata, save_branch_metadata};

use crate::a_1_file_system_hiding::b_1_2_directory_interaction::{
    check_directory, copy_directory, create_directory, delete_directory, rename_directory,
};
use crate::a_1_file_system_hiding::{
    b_1_1_file_interaction::{get_filename, write_file},
    REMOTE,
};

use std::collections::HashMap;
use std::io;

pub fn push(
    path: &str,
    remote_path: &str,
    branch: &str,
    all: bool,
    force: bool,
) -> Result<String, io::Error> {
    if !branch.is_empty() && all {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Cannot push all ('--all') and a specific branch simultaneously. Use either --all or specific a branch.",
        ));
    }

    let local_absolute_path = is_repository(path)?;
    let local_repo_metadata = load_repo_metadata(&local_absolute_path)?;
    let remote_absolute_path = if remote_path == REMOTE {
        is_repository(&format!("{}/{}", local_absolute_path, REMOTE))?
    } else {
        is_repository(remote_path)?
    };
    let mut remote_repo_metadata = load_repo_metadata(&remote_absolute_path)?;
    let mut branches: HashMap<String, String> = HashMap::new();
    let mut push_report = String::new();

    if !all {
        let branch_to_push = if branch.is_empty() {
            local_repo_metadata.head
        } else {
            branch.to_string()
        };

        if !local_repo_metadata.branches.contains_key(&branch_to_push) {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!(
                    "Branch '{}' not found in repository '{}'.",
                    branch_to_push,
                    get_filename(&remote_absolute_path)
                ),
            ));
        }

        branches.insert(
            branch_to_push.to_string(),
            local_repo_metadata.branches[&branch_to_push].clone(),
        );
    } else {
        branches.extend(local_repo_metadata.branches.clone());
    }

    for (branch_to_push, local_last_revision_id) in branches {
        let local_branch_metadata = load_branch_metadata(&local_absolute_path, &branch_to_push)?;

        if !remote_repo_metadata.branches.contains_key(&branch_to_push) {
            init_branch(&remote_absolute_path, &branch_to_push, false)?;
            push_report.push_str(&format!(
                "Branch '{}' created in '{}'.\n",
                branch_to_push,
                get_filename(&remote_absolute_path)
            ));
        }

        if local_last_revision_id.is_empty() {
            push_report.push_str(&format!(
                "No commits in branch '{}' yet...\n",
                branch_to_push
            ));
            continue;
        }

        let mut remote_branch_metadata =
            load_branch_metadata(&remote_absolute_path, &branch_to_push)?;
        let mut extended_commits = Vec::new();

        if check_directory(&format!(
            "{}/.dvcs/origin/{}/temp_commits",
            remote_absolute_path, branch_to_push
        )) {
            delete_directory(
                &format!(
                    "{}/.dvcs/origin/{}/temp_commits",
                    remote_absolute_path, branch_to_push
                ),
                true,
            )?;
        }

        create_directory(&format!(
            "{}/.dvcs/origin/{}/temp_commits",
            remote_absolute_path, branch_to_push
        ))?;

        if let Some(remote_last_revision_id) = remote_branch_metadata.head_commit {
            let local_last_revision_metadata = load_revision_metadata(
                &local_absolute_path,
                &branch_to_push,
                &local_last_revision_id,
            )?;
            let remote_last_revision_metadata = load_revision_metadata(
                &remote_absolute_path,
                &branch_to_push,
                &remote_last_revision_id,
            )?;

            if local_last_revision_id == remote_last_revision_id {
                push_report.push_str(&format!(
                    "Branch '{}' is already up to date.\n",
                    branch_to_push
                ));
                continue;
            } else if !local_last_revision_metadata
                .parents
                .contains(&remote_last_revision_id)
            {
                if !force {
                    return if local_last_revision_metadata.timestamp
                        < remote_last_revision_metadata.timestamp
                    {
                        Err(io::Error::new(
                            io::ErrorKind::Other,
                            format!(
                                "Branch '{}' is ahead of local branch. Please pull changes from '{}' before pushing.",
                                branch_to_push, get_filename(&remote_absolute_path)
                            ),
                        ))
                    } else {
                        Err(io::Error::new(
                            io::ErrorKind::Other,
                            format!(
                                "Cannot push branch '{}': '{}' and '{}' have diverged.",
                                branch_to_push,
                                get_filename(&remote_absolute_path),
                                get_filename(&local_absolute_path)
                            ),
                        ))
                    };
                }

                push_report.push_str(&format!(
                    "Force pushing branch '{}' to overwrite '{}' changes.\n",
                    branch_to_push,
                    get_filename(&remote_absolute_path)
                ));
            }

            let mut commit_to_push = local_branch_metadata.commits.clone();

            match local_branch_metadata
                .commits
                .iter()
                .position(|id| *id == remote_last_revision_id)
            {
                Some(index) => {
                    commit_to_push = commit_to_push[index + 1..].to_vec();
                }
                None => {
                    return Err(io::Error::new(
                        io::ErrorKind::NotFound,
                        format!(
                            "Revision '{}' not found in '{}' branch '{}'.",
                            remote_last_revision_id,
                            get_filename(&local_absolute_path),
                            branch_to_push
                        ),
                    ));
                }
            }

            for revision_id_to_push in commit_to_push.iter() {
                copy_directory(
                    &format!(
                        "{}/.dvcs/origin/{}/commits/{}",
                        local_absolute_path, branch_to_push, revision_id_to_push
                    ),
                    &format!(
                        "{}/.dvcs/origin/{}/temp_commits/{}",
                        remote_absolute_path, branch_to_push, revision_id_to_push
                    ),
                )?;
            }

            extended_commits.extend(commit_to_push);
        } else {
            copy_directory(
                &format!(
                    "{}/.dvcs/origin/{}/commits",
                    local_absolute_path, branch_to_push
                ),
                &format!(
                    "{}/.dvcs/origin/{}/temp_commits",
                    remote_absolute_path, branch_to_push
                ),
            )?;
            extended_commits.extend(local_branch_metadata.commits);
        }

        if !force {
            for extended_revision_id in extended_commits.iter() {
                copy_directory(
                    &format!(
                        "{}/.dvcs/origin/{}/temp_commits/{}",
                        remote_absolute_path, branch_to_push, extended_revision_id
                    ),
                    &format!(
                        "{}/.dvcs/origin/{}/commits/{}",
                        remote_absolute_path, branch_to_push, extended_revision_id
                    ),
                )?;
            }
            delete_directory(
                &format!(
                    "{}/.dvcs/origin/{}/temp_commits",
                    remote_absolute_path, branch_to_push
                ),
                true,
            )?;
            remote_branch_metadata.commits.extend(extended_commits);
        } else {
            delete_directory(
                &format!(
                    "{}/.dvcs/origin/{}/commits",
                    remote_absolute_path, branch_to_push
                ),
                true,
            )?;
            rename_directory(
                &format!(
                    "{}/.dvcs/origin/{}/temp_commits",
                    remote_absolute_path, branch_to_push
                ),
                &format!(
                    "{}/.dvcs/origin/{}/commits",
                    remote_absolute_path, branch_to_push
                ),
            )?;
            remote_branch_metadata.commits = extended_commits;
        }
        remote_branch_metadata.head_commit = Some(local_last_revision_id.clone());
        save_branch_metadata(
            &remote_absolute_path,
            &branch_to_push,
            &remote_branch_metadata,
        )?;

        remote_repo_metadata
            .branches
            .insert(branch_to_push.clone(), local_last_revision_id);

        push_report.push_str(&format!(
            "Branch '{}' pushed successfully.\n",
            branch_to_push
        ));
    }

    let head = remote_repo_metadata.head.clone();
    write_file(
        &format!("{}/.dvcs/HEAD", remote_absolute_path),
        &format!(
            "commit: {}\nref: origin/{}",
            remote_repo_metadata
                .branches
                .get(&head)
                .unwrap_or(&"N/A".to_string()),
            head
        ),
    )?;
    save_repo_metadata(&remote_absolute_path, &remote_repo_metadata)?;
    Ok(push_report)
}

pub fn pull(
    path: &str,
    remote_path: &str,
    branch: &str,
    all: bool,
    force: bool,
) -> Result<String, io::Error> {
    if !branch.is_empty() && all {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Cannot pull all ('--all') and a specific branch simultaneously. Use either --all or specify a branch.",
        ));
    }

    let local_absolute_path = is_repository(path)?;
    let remote_absolute_path = if remote_path == REMOTE {
        is_repository(&format!("{}/{}", local_absolute_path, REMOTE))?
    } else {
        is_repository(remote_path)?
    };
    let mut local_repo_metadata = load_repo_metadata(&local_absolute_path)?;
    let remote_repo_metadata = load_repo_metadata(&remote_absolute_path)?;
    let mut branches: HashMap<String, String> = HashMap::new();
    let mut pull_report = String::new();

    if all {
        branches.extend(remote_repo_metadata.branches.clone());
    } else {
        let branch_to_pull = if branch.is_empty() {
            remote_repo_metadata.head
        } else {
            branch.to_string()
        };

        if !remote_repo_metadata.branches.contains_key(&branch_to_pull) {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!(
                    "Branch '{}' not found in repository '{}'.",
                    branch_to_pull,
                    get_filename(&remote_absolute_path)
                ),
            ));
        }

        branches.insert(
            branch_to_pull.to_string(),
            remote_repo_metadata.branches[&branch_to_pull].clone(),
        );
    }

    for (branch_to_pull, remote_last_revision_id) in branches {
        let remote_branch_metadata = load_branch_metadata(&remote_absolute_path, &branch_to_pull)?;

        if !local_repo_metadata.branches.contains_key(&branch_to_pull) {
            init_branch(&local_absolute_path, &branch_to_pull, false)?;
            pull_report.push_str(&format!(
                "Branch '{}' created in '{}'.\n",
                branch_to_pull,
                get_filename(&local_absolute_path)
            ));
        }

        if remote_last_revision_id.is_empty() {
            pull_report.push_str(&format!(
                "No commits in branch '{}' yet...\n",
                branch_to_pull
            ));
            continue;
        }

        let mut local_branch_metadata =
            load_branch_metadata(&local_absolute_path, &branch_to_pull)?;
        let mut extended_commits = Vec::new();

        if check_directory(&format!(
            "{}/.dvcs/origin/{}/temp_commits",
            local_absolute_path, branch_to_pull
        )) {
            delete_directory(
                &format!(
                    "{}/.dvcs/origin/{}/temp_commits",
                    local_absolute_path, branch_to_pull
                ),
                true,
            )?;
        }

        create_directory(&format!(
            "{}/.dvcs/origin/{}/temp_commits",
            local_absolute_path, branch_to_pull
        ))?;

        if let Some(local_last_revision_id) = local_branch_metadata.head_commit {
            let local_last_revision_metadata = load_revision_metadata(
                &local_absolute_path,
                &branch_to_pull,
                &local_last_revision_id,
            )?;
            let remote_last_revision_metadata = load_revision_metadata(
                &remote_absolute_path,
                &branch_to_pull,
                &remote_last_revision_id,
            )?;

            if local_last_revision_id == remote_last_revision_id {
                pull_report.push_str(&format!(
                    "Branch '{}' is already up to date.\n",
                    branch_to_pull
                ));
                continue;
            } else if !remote_last_revision_metadata
                .parents
                .contains(&local_last_revision_id)
            {
                if !force {
                    return if remote_last_revision_metadata.timestamp
                        < local_last_revision_metadata.timestamp
                    {
                        Err(io::Error::new(
                            io::ErrorKind::Other,
                            format!(
                                "Branch '{}' is ahead of remote branch. Please push changes to '{}' before pulling.",
                                branch_to_pull, get_filename(&remote_absolute_path)
                            ),
                        ))
                    } else {
                        Err(io::Error::new(
                            io::ErrorKind::Other,
                            format!(
                                "Cannot pull branch '{}': '{}' and '{}' have diverged.",
                                branch_to_pull,
                                get_filename(&local_absolute_path),
                                get_filename(&remote_absolute_path)
                            ),
                        ))
                    };
                }

                pull_report.push_str(&format!(
                    "Force pulling branch '{}' to overwrite '{}' changes.\n",
                    branch_to_pull,
                    get_filename(&local_absolute_path)
                ));
            }

            let mut commit_to_pull = remote_branch_metadata.commits.clone();
            println!("commit_to_pull: {:?}", commit_to_pull);

            if let Some(index) = remote_branch_metadata
                .commits
                .iter()
                .position(|id| *id == local_last_revision_id)
            {
                commit_to_pull = commit_to_pull[index + 1..].to_vec();
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!(
                        "Revision '{}' not found in '{}' branch '{}'.",
                        local_last_revision_id,
                        get_filename(&remote_absolute_path),
                        branch_to_pull
                    ),
                ));
            }

            for revision_id_to_pull in commit_to_pull.iter() {
                copy_directory(
                    &format!(
                        "{}/.dvcs/origin/{}/commits/{}",
                        remote_absolute_path, branch_to_pull, revision_id_to_pull
                    ),
                    &format!(
                        "{}/.dvcs/origin/{}/temp_commits/{}",
                        local_absolute_path, branch_to_pull, revision_id_to_pull
                    ),
                )?;
            }

            extended_commits.extend(commit_to_pull);
        } else {
            copy_directory(
                &format!(
                    "{}/.dvcs/origin/{}/commits",
                    remote_absolute_path, branch_to_pull
                ),
                &format!(
                    "{}/.dvcs/origin/{}/temp_commits",
                    local_absolute_path, branch_to_pull
                ),
            )?;
            extended_commits.extend(remote_branch_metadata.commits);
        }

        if force {
            delete_directory(
                &format!(
                    "{}/.dvcs/origin/{}/commits",
                    local_absolute_path, branch_to_pull
                ),
                true,
            )?;
            rename_directory(
                &format!(
                    "{}/.dvcs/origin/{}/temp_commits",
                    local_absolute_path, branch_to_pull
                ),
                &format!(
                    "{}/.dvcs/origin/{}/commits",
                    local_absolute_path, branch_to_pull
                ),
            )?;
            local_branch_metadata.commits = extended_commits;
        } else {
            for extended_revision_id in extended_commits.iter() {
                copy_directory(
                    &format!(
                        "{}/.dvcs/origin/{}/temp_commits/{}",
                        local_absolute_path, branch_to_pull, extended_revision_id
                    ),
                    &format!(
                        "{}/.dvcs/origin/{}/commits/{}",
                        local_absolute_path, branch_to_pull, extended_revision_id
                    ),
                )?;
            }
            delete_directory(
                &format!(
                    "{}/.dvcs/origin/{}/temp_commits",
                    local_absolute_path, branch_to_pull
                ),
                true,
            )?;
            local_branch_metadata.commits.extend(extended_commits);
        }

        local_branch_metadata.head_commit = Some(remote_last_revision_id.clone());
        save_branch_metadata(
            &local_absolute_path,
            &branch_to_pull,
            &local_branch_metadata,
        )?;

        local_repo_metadata
            .branches
            .insert(branch_to_pull.clone(), remote_last_revision_id);

        pull_report.push_str(&format!(
            "Branch '{}' pulled successfully.\n",
            branch_to_pull
        ));
    }

    let head = local_repo_metadata.head.clone();
    write_file(
        &format!("{}/.dvcs/HEAD", local_absolute_path),
        &format!(
            "commit: {}\nref: origin/{}",
            local_repo_metadata
                .branches
                .get(&head)
                .unwrap_or(&"N/A".to_string()),
            head
        ),
    )?;
    save_repo_metadata(&local_absolute_path, &local_repo_metadata)?;
    Ok(pull_report)
}
