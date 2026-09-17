//! One background thread that owns a resource and applies jobs to it in
//! submission order. A burst of queued jobs collapses to the newest (every job
//! is a complete desired state, so an older one is superseded on disk anyway),
//! and `flush` is a drain barrier that returns the result of the most recent
//! job — callers gate on that, never on mere enqueue. Errors are collected for
//! the UI log rather than printed to a console nobody watches.

use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

pub type WorkerResult = Result<(), String>;

enum Msg<J> {
    Job(J),
    Flush(Sender<WorkerResult>),
}

pub struct CoalescingWriter<J> {
    tx: Option<Sender<Msg<J>>>,
    worker: Option<JoinHandle<()>>,
    errors: Arc<Mutex<Vec<String>>>,
}

impl<J: Send + 'static> CoalescingWriter<J> {
    /// `label` names the thread and prefixes the errors handed back by
    /// [`take_errors`](Self::take_errors).
    pub fn spawn(
        label: &'static str,
        mut apply: impl FnMut(J) -> WorkerResult + Send + 'static,
    ) -> Result<Self, String> {
        let (tx, rx) = channel::<Msg<J>>();
        let errors: Arc<Mutex<Vec<String>>> = Arc::default();
        let sink = Arc::clone(&errors);
        let worker = std::thread::Builder::new()
            .name(format!("{label}-writer"))
            .spawn(move || {
                // Result of the latest job, carried across batches so a flush
                // that arrives later still reports an earlier failure.
                let mut last: WorkerResult = Ok(());
                while let Ok(msg) = rx.recv() {
                    let mut latest = None;
                    let mut acks = Vec::new();
                    let mut next = Some(msg);
                    while let Some(m) = next {
                        match m {
                            Msg::Job(job) => latest = Some(job),
                            Msg::Flush(ack) => acks.push(ack),
                        }
                        next = rx.try_recv().ok();
                    }
                    if let Some(job) = latest {
                        last = apply(job);
                        if let Err(e) = &last {
                            if let Ok(mut errs) = sink.lock() {
                                errs.push(format!("[{label}] {e}"));
                            }
                        }
                    }
                    for ack in acks {
                        let _ = ack.send(last.clone());
                    }
                }
            })
            .map_err(|e| format!("failed to spawn the {label} writer: {e}"))?;
        Ok(Self {
            tx: Some(tx),
            worker: Some(worker),
            errors,
        })
    }

    /// `false` = the worker is gone and the job was not queued.
    pub fn submit(&self, job: J) -> bool {
        self.tx
            .as_ref()
            .is_some_and(|tx| tx.send(Msg::Job(job)).is_ok())
    }

    /// Block until every queued job has been applied; the result is that of
    /// the most recent job.
    pub fn flush(&self) -> WorkerResult {
        let Some(tx) = &self.tx else {
            return Ok(());
        };
        let (ack, result) = channel();
        if tx.send(Msg::Flush(ack)).is_err() {
            return Err("writer thread is gone".to_string());
        }
        result
            .recv()
            .unwrap_or_else(|_| Err("writer dropped the flush ack".to_string()))
    }

    /// Errors the worker hit since the last call.
    pub fn take_errors(&self) -> Vec<String> {
        self.errors
            .lock()
            .map(|mut errs| std::mem::take(&mut *errs))
            .unwrap_or_default()
    }
}

impl<J> Drop for CoalescingWriter<J> {
    fn drop(&mut self) {
        if let Some(tx) = self.tx.take() {
            let (ack, result) = channel();
            if tx.send(Msg::Flush(ack)).is_ok() {
                let _ = result.recv();
            }
        }
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn flush_returns_the_latest_result_and_collects_errors() {
        let applied = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&applied);
        let writer = CoalescingWriter::spawn("test", move |job: u32| {
            counter.fetch_add(1, Ordering::SeqCst);
            if job == 0 {
                Err("zero".to_string())
            } else {
                Ok(())
            }
        })
        .unwrap();
        assert!(writer.submit(1));
        assert_eq!(writer.flush(), Ok(()));
        assert!(writer.submit(0));
        assert_eq!(writer.flush(), Err("zero".to_string()));
        assert_eq!(writer.take_errors(), vec!["[test] zero".to_string()]);
        assert!(writer.take_errors().is_empty());
        assert_eq!(applied.load(Ordering::SeqCst), 2);
    }
}
