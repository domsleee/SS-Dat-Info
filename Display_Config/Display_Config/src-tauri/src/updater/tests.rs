use super::*;
use futures_util::FutureExt;

fn with_cache(test: impl FnOnce(&Path)) {
    let dir = std::env::temp_dir().join(format!("ss-update-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let path = dir.join("cache.json");
    test(&path);
    if path.exists() {
        std::fs::remove_file(path).unwrap();
    }
    std::fs::remove_dir(dir).unwrap();
}

#[test]
fn cache_expiry_boundaries() {
    let now = 10_000;
    for (success, attempt, reusable) in [
        (now - CACHE_TTL_SECS + 1, 0, true),
        (now - CACHE_TTL_SECS, 0, false),
        (0, now - FAILURE_RETRY_SECS + 1, true),
        (0, now - FAILURE_RETRY_SECS, false),
        (1, now, true),
        (now + 1, now + 1, false),
        (now + 1, 0, false),
        (0, now + 1, false),
        (0, 0, false),
    ] {
        let cache = UpdateCache {
            checked_at_secs: success,
            last_attempt_secs: attempt,
            latest_version: "0.4.6".into(),
        };
        assert_eq!(
            cache.can_reuse(now),
            reusable,
            "success={success}, attempt={attempt}"
        );
    }
}

#[test]
fn background_uses_cache_but_manual_check_fetches() {
    with_cache(|path| {
        write_cache_at(
            path,
            &UpdateCache {
                checked_at_secs: now_secs(),
                last_attempt_secs: now_secs(),
                latest_version: get_version(),
            },
        )
        .unwrap();
        let info = check_updates(false, path, async { panic!("Fresh cache must skip fetch") })
            .now_or_never()
            .unwrap()
            .unwrap();
        assert_eq!(info.latest_version, get_version());
        let info = check_updates(true, path, async { Ok("99.0.0".into()) })
            .now_or_never()
            .unwrap()
            .unwrap();
        assert_eq!(info.latest_version, "99.0.0");
        let cache = read_cache(path).unwrap();
        assert_eq!(cache.latest_version, "99.0.0");
        assert_eq!(cache.checked_at_secs, cache.last_attempt_secs);
    });
}

#[test]
fn failed_checks_preserve_cache_and_back_off_but_manual_checks_report_errors() {
    with_cache(|path| {
        let previous_success = now_secs() - CACHE_TTL_SECS;
        write_cache_at(
            path,
            &UpdateCache {
                checked_at_secs: previous_success,
                last_attempt_secs: previous_success,
                latest_version: "99.0.0".into(),
            },
        )
        .unwrap();
        let info = check_updates(false, path, async { Err("offline".into()) })
            .now_or_never()
            .unwrap()
            .unwrap();
        assert_eq!(info.latest_version, "99.0.0");
        let cache = read_cache(path).unwrap();
        assert_eq!(cache.checked_at_secs, previous_success);
        assert!(cache.can_reuse(now_secs()));
        check_updates(false, path, async { panic!("Backoff must skip fetch") })
            .now_or_never()
            .unwrap()
            .unwrap();
        let error = check_updates(true, path, async { Err("offline".into()) })
            .now_or_never()
            .unwrap()
            .unwrap_err();
        assert_eq!(error, "Could not check for updates: offline");
        assert_eq!(read_cache(path).unwrap().checked_at_secs, previous_success);
    });
}

#[test]
fn cold_cache_failure_is_quiet_only_for_background_checks() {
    for force in [false, true] {
        with_cache(|path| {
            let result = check_updates(force, path, async { Err("offline".into()) })
                .now_or_never()
                .unwrap();
            if force {
                assert!(result.is_err());
            } else {
                let info = result.unwrap();
                assert_eq!(info.current_version, info.latest_version);
            }
            let cache = read_cache(path).unwrap();
            assert_eq!(cache.checked_at_secs, 0);
            assert!(cache.can_reuse(now_secs()));
        });
    }
}

#[test]
fn future_dated_cache_is_refreshed() {
    with_cache(|path| {
        write_cache_at(
            path,
            &UpdateCache {
                checked_at_secs: now_secs() + 86_400,
                last_attempt_secs: now_secs() + 86_400,
                latest_version: get_version(),
            },
        )
        .unwrap();
        let info = check_updates(false, path, async { Ok("99.0.0".into()) })
            .now_or_never()
            .unwrap()
            .unwrap();
        assert_eq!(info.latest_version, "99.0.0");
        assert!(read_cache(path).unwrap().checked_at_secs <= now_secs());
    });
}

#[test]
fn reads_legacy_cache_and_ignores_broken_json() {
    with_cache(|path| {
        std::fs::write(path, r#"{"checked_at_secs":100,"latest_version":"0.4.6"}"#).unwrap();
        assert_eq!(read_cache(path).unwrap().last_attempt_secs, 0);
        std::fs::write(path, "{broken").unwrap();
        assert!(read_cache(path).is_none());
        let info = check_updates(false, path, async { Ok("99.0.0".into()) })
            .now_or_never()
            .unwrap()
            .unwrap();
        assert_eq!(info.latest_version, "99.0.0");
    });
}

#[test]
fn only_offers_newer_versions() {
    for (current, latest, offered) in [
        ("0.4.5", "0.4.4", false),
        ("0.4.5", "0.4.5", false),
        ("0.4.5", "0.4.6", true),
        ("0.4.9", "0.4.10", true),
        ("0.4.5", "0.4.6-rc.1", true),
        ("0.4.5", "0.4.5-rc.1", false),
        ("0.4.5-rc.1", "0.4.5", true),
        ("0.4.5+build.1", "0.4.5+build.2", false),
        ("0.4.5", "invalid", false),
        ("0.4.5", "", false),
    ] {
        let info = UpdateInfo::new(current.into(), latest.into());
        assert_eq!(info.current_version, current);
        assert_eq!(info.latest_version, if offered { latest } else { current });
    }
}

// Concurrent replacement can briefly deny access on Windows. Retry I/O, but never invalid JSON.
const REPLACEMENT_ERRORS: [i32; 2] = [5, 32];
const REPLACEMENT_RETRIES: usize = 100;

fn retry_through_replacement<T>(mut operation: impl FnMut() -> io::Result<T>) -> io::Result<T> {
    for _ in 0..REPLACEMENT_RETRIES {
        match operation() {
            Err(error)
                if error
                    .raw_os_error()
                    .is_some_and(|code| REPLACEMENT_ERRORS.contains(&code)) =>
            {
                std::thread::sleep(Duration::from_millis(1));
            }
            result => return result,
        }
    }
    operation()
}

#[test]
fn replacement_errors_are_retried_but_other_errors_surface() {
    let mut calls = 0;
    let published = retry_through_replacement(|| {
        calls += 1;
        match calls {
            1 => Err(io::Error::from_raw_os_error(REPLACEMENT_ERRORS[0])),
            2 => Err(io::Error::from_raw_os_error(REPLACEMENT_ERRORS[1])),
            _ => Ok(calls),
        }
    })
    .unwrap();
    assert_eq!((published, calls), (3, 3));

    let mut calls = 0;
    let error = retry_through_replacement(|| {
        calls += 1;
        Err::<(), _>(io::Error::from(io::ErrorKind::NotFound))
    })
    .unwrap_err();
    assert_eq!((error.kind(), calls), (io::ErrorKind::NotFound, 1));

    let mut calls = 0;
    let error = retry_through_replacement(|| {
        calls += 1;
        Err::<(), _>(io::Error::from_raw_os_error(REPLACEMENT_ERRORS[0]))
    })
    .unwrap_err();
    assert_eq!(error.raw_os_error(), Some(REPLACEMENT_ERRORS[0]));
    assert_eq!(calls, REPLACEMENT_RETRIES + 1);
}

#[test]
fn concurrent_writes_publish_complete_caches() {
    let dir = std::env::temp_dir().join(format!("ss-update-{}", uuid::Uuid::new_v4()));
    let path = dir.join("cache.json");
    let barrier = std::sync::Barrier::new(8);
    std::thread::scope(|scope| {
        for writer in 0..8 {
            let path = &path;
            let barrier = &barrier;
            scope.spawn(move || {
                barrier.wait();
                for attempt in 0..32 {
                    let stamp = writer * 32 + attempt;
                    retry_through_replacement(|| {
                        write_cache_at(
                            path,
                            &UpdateCache {
                                checked_at_secs: stamp,
                                last_attempt_secs: stamp,
                                latest_version: format!("0.4.{stamp}"),
                            },
                        )
                    })
                    .unwrap();
                    let published = retry_through_replacement(|| std::fs::read(path)).unwrap();
                    let cache: UpdateCache = serde_json::from_slice(&published).unwrap();
                    assert_eq!(cache.checked_at_secs, cache.last_attempt_secs);
                    assert_eq!(
                        cache.latest_version,
                        format!("0.4.{}", cache.checked_at_secs)
                    );
                }
            });
        }
    });
    assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 1);
    std::fs::remove_file(path).unwrap();
    std::fs::remove_dir(dir).unwrap();
}

#[test]
fn failed_publish_removes_temporary_file() {
    let dir = std::env::temp_dir().join(format!("ss-update-{}", uuid::Uuid::new_v4()));
    let path = dir.join("cache.json");
    std::fs::create_dir_all(&path).unwrap();
    assert!(
        write_cache_at(
            &path,
            &UpdateCache {
                checked_at_secs: 1,
                last_attempt_secs: 1,
                latest_version: "0.4.5".into(),
            },
        )
        .is_err()
    );
    assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 1);
    std::fs::remove_dir(path).unwrap();
    std::fs::remove_dir(dir).unwrap();
}
