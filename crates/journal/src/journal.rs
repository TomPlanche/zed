use chrono::{Datelike, Local, NaiveTime, Timelike};
use editor::scroll::Autoscroll;
use editor::{Editor, SelectionEffects};
use gpui::{App, AppContext as _, Context, TaskExt, Window, actions};
pub use settings::HourFormat;
use settings::{RegisterSetting, Settings, SettingsLocation};
use std::{
    fs::OpenOptions,
    path::{Path, PathBuf},
    sync::Arc,
};
use util::rel_path::RelPath;
use workspace::{AppState, OpenResult, OpenVisible, Workspace};

actions!(
    journal,
    [
        /// Creates a new journal entry for today.
        NewJournalEntry
    ]
);

/// Settings specific to journaling
#[derive(Clone, Debug, RegisterSetting)]
pub struct JournalSettings {
    /// The path of the directory where journal entries are stored.
    ///
    /// Default: `~`
    pub path: String,
    /// What format to display the hours in.
    ///
    /// Default: hour12
    pub hour_format: HourFormat,
}

impl settings::Settings for JournalSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let journal = content.project.journal.clone().unwrap();

        Self {
            path: journal.path.unwrap(),
            hour_format: journal.hour_format.unwrap(),
        }
    }
}

pub fn init(_: Arc<AppState>, cx: &mut App) {
    cx.observe_new(
        |workspace: &mut Workspace, _window, _cx: &mut Context<Workspace>| {
            workspace.register_action(|workspace, _: &NewJournalEntry, window, cx| {
                new_journal_entry(workspace, window, cx);
            });
        },
    )
    .detach();
}

pub fn new_journal_entry(workspace: &Workspace, window: &mut Window, cx: &mut App) {
    // `journal: new journal entry` is a global command, not tied to a buffer, so
    // resolve the setting against the focused item's worktree to pick which
    // project's local `.zed/settings.json` applies. When nothing is focused (e.g.
    // an empty window or a non-file pane), fall back to the first project root,
    // and to user/global settings when no worktree is open at all.
    let active_project_path = workspace
        .active_item(cx)
        .and_then(|item| item.project_path(cx));
    let location = match &active_project_path {
        Some(project_path) => Some(SettingsLocation {
            worktree_id: project_path.worktree_id,
            path: project_path.path.as_ref(),
        }),
        None => workspace
            .visible_worktrees(cx)
            .next()
            .map(|worktree| SettingsLocation {
                worktree_id: worktree.read(cx).id(),
                path: RelPath::empty(),
            }),
    };
    let settings = JournalSettings::get(location, cx);
    let journal_dir = match journal_dir(&settings.path) {
        Some(journal_dir) => journal_dir,
        None => {
            log::error!("Can't determine journal directory");
            return;
        }
    };
    let journal_dir_clone = journal_dir.clone();

    let now = Local::now();
    let month_dir = journal_dir
        .join(format!("{:02}", now.year()))
        .join(format!("{:02}", now.month()));
    let entry_path = month_dir.join(format!("{:02}.md", now.day()));
    let now = now.time();
    let entry_heading = heading_entry(now, &settings.hour_format);

    let create_entry = cx.background_spawn(async move {
        std::fs::create_dir_all(month_dir)?;
        OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open(&entry_path)?;
        Ok::<_, std::io::Error>((journal_dir, entry_path))
    });

    let worktrees = workspace.visible_worktrees(cx).collect::<Vec<_>>();
    let mut open_new_workspace = true;
    'outer: for worktree in worktrees.iter() {
        let worktree_root = worktree.read(cx).abs_path();
        if *worktree_root == journal_dir_clone {
            open_new_workspace = false;
            break;
        }
        for directory in worktree.read(cx).directories(true, 1) {
            let full_directory_path = worktree_root.join(directory.path.as_std_path());
            if full_directory_path.ends_with(&journal_dir_clone) {
                open_new_workspace = false;
                break 'outer;
            }
        }
    }

    let app_state = workspace.app_state().clone();
    let view_snapshot = workspace.weak_handle();

    window
        .spawn(cx, async move |cx| {
            let (journal_dir, entry_path) = create_entry.await?;
            let opened = if open_new_workspace {
                let OpenResult {
                    window: new_workspace,
                    ..
                } = cx
                    .update(|_window, cx| {
                        workspace::open_paths(
                            &[journal_dir],
                            app_state,
                            workspace::OpenOptions::default(),
                            cx,
                        )
                    })?
                    .await?;
                new_workspace
                    .update(cx, |multi_workspace, window, cx| {
                        let workspace = multi_workspace.workspace().clone();
                        workspace.update(cx, |workspace, cx| {
                            workspace.open_paths(
                                vec![entry_path],
                                workspace::OpenOptions {
                                    visible: Some(OpenVisible::All),
                                    ..Default::default()
                                },
                                None,
                                window,
                                cx,
                            )
                        })
                    })?
                    .await
            } else {
                view_snapshot
                    .update_in(cx, |workspace, window, cx| {
                        workspace.open_paths(
                            vec![entry_path],
                            workspace::OpenOptions {
                                visible: Some(OpenVisible::All),
                                ..Default::default()
                            },
                            None,
                            window,
                            cx,
                        )
                    })?
                    .await
            };

            if let Some(Some(Ok(item))) = opened.first()
                && let Some(editor) = item.downcast::<Editor>().map(|editor| editor.downgrade())
            {
                editor.update_in(cx, |editor, window, cx| {
                    let len = editor.buffer().read(cx).len(cx);
                    editor.change_selections(
                        SelectionEffects::scroll(Autoscroll::center()),
                        window,
                        cx,
                        |s| s.select_ranges([len..len]),
                    );
                    if len.0 > 0 {
                        editor.insert("\n\n", window, cx);
                    }
                    editor.insert(&entry_heading, window, cx);
                    editor.insert("\n\n", window, cx);
                })?;
            }

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
}

fn journal_dir(path: &str) -> Option<PathBuf> {
    let expanded = shellexpand::full(path).ok()?;
    let base_path = Path::new(expanded.as_ref());
    let absolute_path = if base_path.is_absolute() {
        base_path.to_path_buf()
    } else {
        log::warn!("Invalid journal path {path:?} (not absolute), falling back to home directory",);
        std::env::home_dir()?
    };
    Some(absolute_path.join("journal"))
}

fn heading_entry(now: NaiveTime, hour_format: &HourFormat) -> String {
    match hour_format {
        HourFormat::Hour24 => {
            let hour = now.hour();
            format!("# {}:{:02}", hour, now.minute())
        }
        HourFormat::Hour12 => {
            let (pm, hour) = now.hour12();
            let am_or_pm = if pm { "PM" } else { "AM" };
            format!("# {}:{:02} {}", hour, now.minute(), am_or_pm)
        }
    }
}

#[cfg(test)]
mod tests {
    mod heading_entry_tests {
        use super::super::*;

        #[test]
        fn test_heading_entry_defaults_to_hour_12() {
            let naive_time = NaiveTime::from_hms_milli_opt(15, 0, 0, 0).unwrap();
            let actual_heading_entry = heading_entry(naive_time, &HourFormat::Hour12);
            let expected_heading_entry = "# 3:00 PM";

            assert_eq!(actual_heading_entry, expected_heading_entry);
        }

        #[test]
        fn test_heading_entry_is_hour_12() {
            let naive_time = NaiveTime::from_hms_milli_opt(15, 0, 0, 0).unwrap();
            let actual_heading_entry = heading_entry(naive_time, &HourFormat::Hour12);
            let expected_heading_entry = "# 3:00 PM";

            assert_eq!(actual_heading_entry, expected_heading_entry);
        }

        #[test]
        fn test_heading_entry_is_hour_24() {
            let naive_time = NaiveTime::from_hms_milli_opt(15, 0, 0, 0).unwrap();
            let actual_heading_entry = heading_entry(naive_time, &HourFormat::Hour24);
            let expected_heading_entry = "# 15:00";

            assert_eq!(actual_heading_entry, expected_heading_entry);
        }
    }

    mod journal_dir_tests {
        use super::super::*;

        #[test]
        #[cfg(target_family = "unix")]
        fn test_absolute_unix_path() {
            let result = journal_dir("/home/user");
            assert!(result.is_some());
            let path = result.unwrap();
            assert!(path.is_absolute());
            assert_eq!(path, PathBuf::from("/home/user/journal"));
        }

        #[test]
        fn test_tilde_expansion() {
            let result = journal_dir("~/documents");
            assert!(result.is_some());
            let path = result.unwrap();

            assert!(path.is_absolute(), "Tilde should expand to absolute path");

            if let Some(home) = std::env::home_dir() {
                assert_eq!(path, home.join("documents").join("journal"));
            }
        }

        #[test]
        fn test_relative_path_falls_back_to_home() {
            for relative_path in ["relative/path", "NONEXT/some/path", "../some/path"] {
                let result = journal_dir(relative_path);
                assert!(result.is_some(), "Failed for path: {}", relative_path);
                let path = result.unwrap();

                assert!(
                    path.is_absolute(),
                    "Path should be absolute for input '{}', got: {:?}",
                    relative_path,
                    path
                );

                if let Some(home) = std::env::home_dir() {
                    assert_eq!(
                        path,
                        home.join("journal"),
                        "Should fall back to home directory for input '{}'",
                        relative_path
                    );
                }
            }
        }

        #[test]
        #[cfg(target_os = "windows")]
        fn test_absolute_path_windows_style() {
            let result = journal_dir("C:\\Users\\user\\Documents");
            assert!(result.is_some());
            let path = result.unwrap();
            assert_eq!(path, PathBuf::from("C:\\Users\\user\\Documents\\journal"));
        }
    }

    mod journal_settings_tests {
        use super::super::{HourFormat, JournalSettings};
        use settings::{
            LocalSettingsKind, LocalSettingsPath, SettingsLocation, SettingsStore, WorktreeId,
            default_settings,
        };
        use util::rel_path::rel_path;

        // Both journal settings live in `ProjectSettingsContent`, so a worktree's
        // `.zed/settings.json` must be able to override each of them, and a partial
        // override must fall back to the user/global value for the fields it omits.
        #[gpui::test]
        fn test_journal_settings_are_project_local(cx: &mut gpui::App) {
            let mut store = SettingsStore::new(cx, &default_settings());
            store.register_setting::<JournalSettings>();

            store
                .set_user_settings(
                    r#"{ "journal": { "path": "~/user-journal", "hour_format": "hour24" } }"#,
                    cx,
                )
                .unwrap();

            let global = store.get::<JournalSettings>(None);
            assert_eq!(global.path, "~/user-journal");
            assert_eq!(global.hour_format, HourFormat::Hour24);

            let worktree_id = WorktreeId::from_usize(1);
            // `project_a` overrides both fields.
            store
                .set_local_settings(
                    worktree_id,
                    LocalSettingsPath::InWorktree(rel_path("project_a").into()),
                    LocalSettingsKind::Settings,
                    Some(r#"{ "journal": { "path": "/tmp/journal-a", "hour_format": "hour12" } }"#),
                    cx,
                )
                .unwrap();
            // `project_b` overrides only `hour_format`, leaving `path` inherited.
            store
                .set_local_settings(
                    worktree_id,
                    LocalSettingsPath::InWorktree(rel_path("project_b").into()),
                    LocalSettingsKind::Settings,
                    Some(r#"{ "journal": { "hour_format": "hour12" } }"#),
                    cx,
                )
                .unwrap();

            let in_project_a = store.get::<JournalSettings>(Some(SettingsLocation {
                worktree_id,
                path: rel_path("project_a/entry.md"),
            }));
            assert_eq!(in_project_a.path, "/tmp/journal-a");
            assert_eq!(in_project_a.hour_format, HourFormat::Hour12);

            let in_project_b = store.get::<JournalSettings>(Some(SettingsLocation {
                worktree_id,
                path: rel_path("project_b/entry.md"),
            }));
            assert_eq!(
                in_project_b.path, "~/user-journal",
                "path should fall back to the user setting when only hour_format is overridden"
            );
            assert_eq!(in_project_b.hour_format, HourFormat::Hour12);

            // A path with no local settings falls back to the user/global values.
            let unscoped = store.get::<JournalSettings>(Some(SettingsLocation {
                worktree_id,
                path: rel_path("elsewhere/entry.md"),
            }));
            assert_eq!(unscoped.path, "~/user-journal");
            assert_eq!(unscoped.hour_format, HourFormat::Hour24);
        }
    }
}
