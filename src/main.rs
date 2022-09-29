use cli_table::Color::{Green, Blue, Yellow};

use std::{fs, process::{Command, Output}};

use cli_table::{print_stdout, Cell, Style, Table};


#[derive(Table)]
struct Repo {
    #[table(title = "Repo")]
    #[table(color = "Blue")]
    repo: String,
    #[table(title = "Branch name")]
    #[table(color = "Green")]
    branch_name: String,
    #[table(title = "Diff")]
    #[table(color = "Yellow")]
    diff: String
}

fn main() {
    let table = build_table_from_repos().table()
                                        .title(vec![
                                            "Repo".cell().bold(true),
                                            "Current branch".cell().bold(true),
                                            "Diff".cell().bold(true),
                                        ])
                                        .bold(true);
    assert!(print_stdout(table).is_ok());
}

fn build_table_from_repos() -> Vec<Repo> {
    let mut table = vec![];
    let paths = fs::read_dir(fetch_git_home()).unwrap();

    for path in paths {
        let path_name = path.unwrap().path();
        let path_name_str = path_name.to_str().unwrap();
        let repo_name = path_name.to_str().unwrap().split("/").last().unwrap();
        let branch = current_branch_for_repo(path_name_str);
        let branch_name = String::from_utf8(branch.stdout).unwrap();
        let diff = String::from_utf8(diff_for_repo_branch(path_name_str).stdout).unwrap();
        let diff_empty = diff.is_empty();

        let repo = Repo {
            repo: repo_name.to_string(),
            branch_name,
            diff: if diff_empty { "None".to_string() } else { diff.to_string() }
        };

        table.push(repo);
    }

    return table;
}

fn current_branch_for_repo(repo_path: &str) ->  Output {
    let res = Command::new("git").args(["-C", repo_path, "rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .expect("Error when determining branch");

    return res
}

fn diff_for_repo_branch(repo_path: &str) ->  Output {
    let res = Command::new("git").args(["-C", repo_path, "diff", "--shortstat"])
        .output()
        .expect("Error when fetching git diff");

    return res
}

fn fetch_git_home() -> String {
    return std::env::var("GIT_HOME").expect("ENSURE $GIT_HOME is set.");
}
